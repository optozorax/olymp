use std::borrow::Cow;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Tokens<'a>(pub Vec<&'a [u8]>);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Lines<'a>(pub Vec<Tokens<'a>>);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Test<'a> {
    SimpleInput {
        input: Cow<'a, [u8]>,
    },
    SimpleOutput {
        input: Cow<'a, [u8]>,
        output: Cow<'a, [u8]>,
    },
    WithCheckerInput {
        checker_input: Cow<'a, [u8]>,
    },
    WithCheckerOutput {
        checker_input: Cow<'a, [u8]>,
        checker_output: Cow<'a, [u8]>,
    },
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum TokenizedTest<'a> {
    Simple {
        input: &'a [u8],
        output: Option<Lines<'a>>,
    },
    WithChecker {
        checker_input: &'a [u8],
        checker_output: Option<Lines<'a>>,
    },
}

impl<'a> From<&'a [u8]> for Tokens<'a> {
    fn from(input: &'a [u8]) -> Tokens<'a> {
        Tokens(
            input
                .split(|x| x.is_ascii_whitespace())
                .filter(|x| !x.is_empty())
                .collect(),
        )
    }
}

impl<'a> From<&'a [u8]> for Lines<'a> {
    fn from(input: &'a [u8]) -> Lines<'a> {
        Lines(
            input
                .split(|x| *x == b'\n')
                .filter_map(|x| {
                    // Trim
                    let start = x
                        .iter()
                        .position(|x| !x.is_ascii_whitespace())
                        .unwrap_or(x.len());
                    let end = x.len()
                        - x.iter()
                            .rev()
                            .position(|x| !x.is_ascii_whitespace())
                            .unwrap_or(x.len());
                    if start < end {
                        Some(&x[start..end])
                    } else {
                        None
                    }
                })
                .map(Tokens::from)
                .collect(),
        )
    }
}

impl fmt::Display for Tokens<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            crate::display::Joined {
                elements: self.0.iter().map(|x| std::str::from_utf8(x).unwrap()),
                by: " ",
            }
        )
    }
}

pub trait SplitNotDoubleTrait<T> {
    fn split_not_double<F: FnMut(&T) -> bool>(&self, pred: F) -> SplitNotDouble<T, F>;
}

impl<T> SplitNotDoubleTrait<T> for [T] {
    fn split_not_double<F: FnMut(&T) -> bool>(&self, pred: F) -> SplitNotDouble<T, F> {
        SplitNotDouble::new(self, pred)
    }
}

impl<'a, T: 'a, P: FnMut(&T) -> bool> SplitNotDouble<'a, T, P> {
    #[inline]
    pub(super) fn new(slice: &'a [T], pred: P) -> Self {
        Self {
            v: slice,
            pred,
            finished: false,
        }
    }
}

pub struct SplitNotDouble<'a, T: 'a, P>
where
    P: FnMut(&T) -> bool,
{
    v: &'a [T],
    pred: P,
    finished: bool,
}

impl<'a, T, P> Iterator for SplitNotDouble<'a, T, P>
where
    P: FnMut(&T) -> bool,
    T: std::fmt::Debug,
{
    type Item = &'a [T];

    #[inline]
    fn next(&mut self) -> Option<&'a [T]> {
        if self.finished {
            return None;
        }

        let mut skipped = 0;

        loop {
            match self.v.iter().skip(skipped).position(|x| (self.pred)(x)) {
                None => {
                    self.finished = true;
                    break Some(self.v);
                }
                Some(idx) => match self.v.get(skipped + idx + 1) {
                    Some(elem) => {
                        if (self.pred)(elem) {
                            skipped += idx + 2;
                        } else {
                            let ret = Some(&self.v[..skipped + idx]);
                            self.v = &self.v[skipped + idx + 1..];
                            break ret;
                        }
                    }
                    None => {
                        let ret = Some(&self.v[..skipped + idx]);
                        self.v = &self.v[skipped + idx + 1..];
                        break ret;
                    }
                },
            }
        }
    }
}

fn cow_unescape(input: &[u8]) -> Cow<[u8]> {
    let check = |x: &u8| *x == b'~' || *x == b'%' || *x == b'\\';
    if input.iter().any(|x| check(x)) {
        let mut result = Vec::with_capacity(input.len());
        let mut previous = b' ';
        for i in input {
            if check(i) && previous == *i {
                previous = b' ';
            } else {
                result.push(*i);
                previous = *i;
            }
        }
        Cow::Owned(result)
    } else {
        Cow::Borrowed(input)
    }
}

pub fn read_tests(file: &[u8]) -> Result<Vec<Test>, usize> {
    let mut result = Vec::new();
    for (test_no, test) in file.split_not_double(|x| *x == b'\\').enumerate() {
        let mut iter = test.split_not_double(|x| *x == b'~');
        let first = (iter.next(), iter.next(), iter.next());

        let mut iter_checker = test.split_not_double(|x| *x == b'%');
        let second = (
            iter_checker.next(),
            iter_checker.next(),
            iter_checker.next(),
        );
        match (first, second) {
            ((Some(input), Some(output), None), (Some(_), None, None)) => {
                if output.is_empty() {
                    result.push(Test::SimpleInput {
                        input: cow_unescape(input),
                    });
                } else {
                    result.push(Test::SimpleOutput {
                        input: cow_unescape(input),
                        output: cow_unescape(output),
                    });
                }
            }
            ((Some(_), None, None), (Some(input), Some(output), None)) => {
                if output.is_empty() {
                    result.push(Test::WithCheckerInput {
                        checker_input: cow_unescape(input),
                    });
                } else {
                    result.push(Test::WithCheckerOutput {
                        checker_input: cow_unescape(input),
                        checker_output: cow_unescape(output),
                    });
                }
            }
            ((Some(_), None, None), (Some(_), None, None)) => {
                result.push(Test::SimpleInput {
                    input: cow_unescape(test),
                });
            }
            _ => {
                return Err(test_no);
            }
        }
    }
    Ok(result)
}

pub fn tokenize_tests<'a>(tests: &'a [Test<'a>]) -> Vec<TokenizedTest<'a>> {
    tests
        .iter()
        .map(|x| match x {
            Test::SimpleInput { input } => TokenizedTest::Simple {
                input: input.as_ref(),
                output: None,
            },
            Test::SimpleOutput { input, output } => {
                let lines = Lines::from(output.as_ref());
                if lines.0.is_empty() {
                    TokenizedTest::Simple {
                        input: input.as_ref(),
                        output: None,
                    }
                } else {
                    TokenizedTest::Simple {
                        input: input.as_ref(),
                        output: Some(lines),
                    }
                }
            }
            Test::WithCheckerInput { checker_input } => TokenizedTest::WithChecker {
                checker_input: checker_input.as_ref(),
                checker_output: None,
            },
            Test::WithCheckerOutput {
                checker_input,
                checker_output,
            } => {
                let lines = Lines::from(checker_output.as_ref());
                if lines.0.is_empty() {
                    TokenizedTest::WithChecker {
                        checker_input: checker_input.as_ref(),
                        checker_output: None,
                    }
                } else {
                    TokenizedTest::WithChecker {
                        checker_input: checker_input.as_ref(),
                        checker_output: Some(lines),
                    }
                }
            }
        })
        .collect()
}

pub fn has_checker(tests: &[TokenizedTest<'_>]) -> bool {
    tests
        .iter()
        .any(|x| matches!(x, TokenizedTest::WithChecker { .. }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_not_double() {
        let should_be: Vec<&'static [u8]> = vec![b"0~~1~~~~2~~", b"a~~z"];
        let collected = b"0~~1~~~~2~~~a~~z"
            .split_not_double(|x| *x == b'~')
            .collect::<Vec<_>>();
        assert_eq!(should_be, collected);

        let should_be: Vec<&'static [u8]> = vec![b"0~~1~~~~2~~a~~z"];
        let collected = b"0~~1~~~~2~~a~~z"
            .split_not_double(|x| *x == b'~')
            .collect::<Vec<_>>();
        assert_eq!(should_be, collected);

        let should_be: Vec<&'static [u8]> = vec![b"0", b"1", b"2"];
        let collected = b"0~1~2"
            .split_not_double(|x| *x == b'~')
            .collect::<Vec<_>>();
        assert_eq!(should_be, collected);

        let should_be: Vec<&'static [u8]> = b"~".split(|x| *x == b'~').collect::<Vec<_>>();
        let collected = b"~".split_not_double(|x| *x == b'~').collect::<Vec<_>>();
        assert_eq!(should_be, collected);

        let should_be: Vec<&'static [u8]> = vec![b"~~"];
        let collected = b"~~".split_not_double(|x| *x == b'~').collect::<Vec<_>>();
        assert_eq!(should_be, collected);

        let should_be: Vec<&'static [u8]> = b"".split(|_| true).collect::<Vec<_>>();
        let collected = b"".split_not_double(|x| *x == b'~').collect::<Vec<_>>();
        assert_eq!(should_be, collected);
    }

    #[test]
    fn unescape() {
        assert_eq!(cow_unescape(b"~~%%x~~~~%%1%%").as_ref(), b"~%x~~%1%");
    }

    #[test]
    fn read() {
        let a = read_tests(
            b"1~\\2%\\0%%1 2 3 4~~5 ~3\\5 6 7%\n  \n   \r\t   \\1%abc def      xyz\n1\n\r  2\t3  ",
        )
        .unwrap();
        assert_eq!(
            a,
            vec![
                Test::SimpleInput {
                    input: Cow::Borrowed(b"1"),
                },
                Test::WithCheckerInput {
                    checker_input: Cow::Borrowed(b"2"),
                },
                Test::SimpleOutput {
                    input: Cow::Owned(b"0%1 2 3 4~5 ".to_vec()),
                    output: Cow::Borrowed(b"3"),
                },
                Test::WithCheckerOutput {
                    checker_input: Cow::Borrowed(b"5 6 7"),
                    checker_output: Cow::Borrowed(b"\n  \n   \r\t   "),
                },
                Test::WithCheckerOutput {
                    checker_input: Cow::Borrowed(b"1"),
                    checker_output: Cow::Borrowed(b"abc def      xyz\n1\n\r  2\t3  "),
                },
            ]
        );

        let b = tokenize_tests(&a);
        assert_eq!(
            b,
            vec![
                TokenizedTest::Simple {
                    input: b"1",
                    output: None,
                },
                TokenizedTest::WithChecker {
                    checker_input: b"2",
                    checker_output: None,
                },
                TokenizedTest::Simple {
                    input: b"0%1 2 3 4~5 ",
                    output: Some(Lines(vec![Tokens(vec![b"3"])])),
                },
                TokenizedTest::WithChecker {
                    checker_input: b"5 6 7",
                    checker_output: None,
                },
                TokenizedTest::WithChecker {
                    checker_input: b"1",
                    checker_output: Some(Lines(vec![
                        Tokens(vec![b"abc", b"def", b"xyz"]),
                        Tokens(vec![b"1"]),
                        Tokens(vec![b"2", b"3"])
                    ])),
                },
            ]
        );
    }
}
