/*****************************************************************************
* Generated and tested by: olytest    (https://github.com/optozorax/olytest) *
* Author: Ilya Sheprut                                      a.k.a. optozorax *
* Generated at:                              Tue, 29 Dec 2020 00:21:12 +0700 *
* License: MIT/Apache 2.0                                                    *
*****************************************************************************/

fn solve(a: &[u64]) -> Option<usize> {
	for i in 0..a.len() - 1 {
		let result = a[i] ^ a[i + 1];
		if i.checked_sub(1)
			.and_then(|x| a.get(x))
			.map(|x| *x > result)
			.unwrap_or(false)
			|| a.get(i + 2).map(|x| *x < result).unwrap_or(false)
		{
			return Some(1);
		}
	}

	// else size of array is 64

	let mut answer: Option<usize> = None;
	for i in 0..a.len() {
		for j in i + 1..=a.len() {
			let xored = a[i..j].iter().fold(1, |acc, x| acc ^ x) ^ 1;

			let mut result1 = None;
			for i1 in j..a.len() {
				for j1 in i1 + 1..=a.len() {
					let xored1 = a[i1..j1].iter().fold(1, |acc, x| acc ^ x) ^ 1;
					if xored > xored1 {
						result1 = match result1 {
							Some(a) => Some(std::cmp::min(a, j1 - i1 - 1)),
							None => Some(j1 - i1 - 1),
						};
					}
				}
			}

			if let Some(sum) = result1 {
				answer = match answer {
					Some(a) => Some(std::cmp::min(a, j - i - 1 + sum)),
					None => Some(j - i - 1 + sum),
				};
			}

			let mut result2 = None;
			for i1 in 0..i {
				for j1 in i1 + 1..=i {
					let xored1 = a[i1..j1].iter().fold(1, |acc, x| acc ^ x) ^ 1;
					if xored < xored1 {
						result2 = match result2 {
							Some(a) => Some(std::cmp::min(a, j1 - i1 - 1)),
							None => Some(j1 - i1 - 1),
						};
					}
				}
			}

			if let Some(sum) = result2 {
				answer = match answer {
					Some(a) => Some(std::cmp::min(a, j - i - 1 + sum)),
					None => Some(j - i - 1 + sum),
				};
			}
		}
	}

	answer
}

pub fn main() {
	// ----------------------------- Fast IO ------------------------------ //
	let stdout = stdout();
	let mut writer = BufWriter::new(stdout.lock());
	macro_rules! println { ($($x:tt)*) => { writeln!(writer, $($x)*).unwrap() }; }

	let input = stdin();
	let mut scanner = Scanner::new(input.lock().bytes().map(|x| x.unwrap()));
	#[rustfmt::skip] macro_rules! read { ($t:tt) => { scanner.read::<$t>() }; }
	#[rustfmt::skip] macro_rules! readln { ($t:tt) => { scanner.readln::<$t>() }; }
	// -------------------------------------------------------------------- //

	let _n = read!(usize);
	let a = readln!(u64);
	if let Some(result) = solve(&a) {
		println!("{}", result);
	} else {
		println!("-1");
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

use std::{
    cmp::{max, min, Ordering, Reverse},
    collections::{BTreeMap, BTreeSet, BinaryHeap, VecDeque},
    convert::{identity, TryFrom, TryInto},
    error,
    fmt::{Debug, Display, Error, Formatter},
    io::{self, stdin, stdout, BufRead, BufWriter, Read, Write},
    iter::{once, FromIterator, Peekable},
    marker::PhantomData,
    mem::swap,
    ops::{Add, Div, Mul, Neg, Range, RangeInclusive, Rem, Sub},
    str::FromStr,
};

//----------------------------------------------------------------------------

fn is_ascii_newline(byte: u8) -> bool { byte == b'\n' || byte == b'\r' }

struct Scanner<I: Iterator> {
    iter: Peekable<I>,
    buf: Vec<u8>,
}

impl<I: Iterator<Item = u8>> Scanner<I> {
    pub fn new(iter: I) -> Self { Self { iter: iter.peekable(), buf: Vec::with_capacity(100) } }

    pub fn read<T: FromStr>(&mut self) -> T
    where T::Err: Debug {
        self.buf.clear();
        self.skip_spaces();
        while let Some(byte) = self.iter.peek().copied().filter(|x| !x.is_ascii_whitespace()) {
            self.buf.push(byte);
            self.iter.next();
        }
        let s = std::str::from_utf8(&self.buf).unwrap_or_else(|_| panic!("input is not utf8: {:?}", self.buf));
        T::from_str(s).unwrap_or_else(|err| panic!("cannot parse from {:?}, error: {:?}", s, err))
    }

    pub fn byte(&mut self) -> u8 { self.iter.next().unwrap() }

    pub fn bytes(&mut self) -> Vec<u8> {
        let mut result = Vec::new();
        self.skip_one_newline();
        while let Some(byte) = self.iter.peek().copied().filter(|x| !is_ascii_newline(*x)) {
            self.iter.next();
            result.push(byte);
        }
        result
    }

    pub fn readln<T: FromStr>(&mut self) -> Vec<T>
    where T::Err: Debug {
        self.skip_one_newline();
        let mut result = Vec::new();
        self.skip_spaces_but_not_newlines();
        while self.iter.peek().map(|x| !is_ascii_newline(*x)).unwrap_or(false) {
            result.push(self.read());
            self.skip_spaces_but_not_newlines();
        }
        result
    }

    fn skip_spaces_but_not_newlines(&mut self) {
        while self.iter.peek().map(|x| x.is_ascii_whitespace() && !is_ascii_newline(*x)).unwrap_or(false) {
            self.iter.next();
        }
    }

    fn skip_spaces(&mut self) {
        while self.iter.peek().map(|x| x.is_ascii_whitespace()).unwrap_or(false) {
            self.iter.next();
        }
    }

    fn skip_one_newline(&mut self) {
        if self.iter.peek().copied().map(|x| x == b'\r').unwrap_or(false) {
            self.iter.next();
            if self.iter.peek().copied().map(|x| x == b'\n').unwrap_or(false) {
                self.iter.next();
            }
        }
        if self.iter.peek().copied().map(|x| x == b'\n').unwrap_or(false) {
            self.iter.next();
        }
    }
}

