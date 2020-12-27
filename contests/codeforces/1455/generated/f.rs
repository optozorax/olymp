/*****************************************************************************
* Generated and tested by: olytest    (https://github.com/optozorax/olytest) *
* Author: Ilya Sheprut                                      a.k.a. optozorax *
* Generated at:                              Mon, 28 Dec 2020 04:53:32 +0700 *
* License: MIT/Apache 2.0                                                    *
*****************************************************************************/

#[derive(Default, Clone, Debug)]
struct StringActions {
	free: bool,
	letter_determined: BTreeMap<u8, BTreeSet<LetterDetermined>>,
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Ord, Eq)]
enum LetterDetermined {
	OneAction,
	TwoActions(u8),
	ThreeActions(u8, u8),
}

fn solve<B: Fn(u8) -> u8>(input: &[u8], better: B) -> Vec<u8> {
	use LetterDetermined::*;
	let mut actions: Vec<StringActions> = vec![StringActions::default(); input.len() + 2];
	actions[0].free = true;
	let mut result = Vec::new();
	for i in 0..input.len() {
		if actions[i].free {
			let current_letter = input[i];
			// B
			actions[i]
				.letter_determined
				.entry(better(current_letter))
				.or_insert_with(BTreeSet::new)
				.insert(OneAction);
			if i + 1 < input.len() {
				let next_letter = input[i + 1];
				// BL
				actions[i]
					.letter_determined
					.entry(next_letter)
					.or_insert_with(BTreeSet::new)
					.insert(TwoActions(better(current_letter)));
				// RB
				actions[i]
					.letter_determined
					.entry(better(next_letter))
					.or_insert_with(BTreeSet::new)
					.insert(TwoActions(current_letter));
				if i + 2 < input.len() {
					let next_next_letter = input[i + 2];
					// BRL
					actions[i]
						.letter_determined
						.entry(next_next_letter)
						.or_insert_with(BTreeSet::new)
						.insert(ThreeActions(better(current_letter), next_letter));
				}
			}
		}
		let mut current = BTreeMap::new();
		std::mem::swap(&mut current, &mut actions[i].letter_determined);
		let (best_letter, best_determined) = current.into_iter().next().unwrap();
		result.push(best_letter);
		for determined in best_determined {
			match determined {
				OneAction => actions[i + 1].free = true,
				TwoActions(letter) => {
					actions[i + 1]
						.letter_determined
						.entry(letter)
						.or_insert_with(BTreeSet::new)
						.insert(OneAction);
					if i + 2 < input.len() {
						let next_letter = input[i + 2];
						// ?L
						actions[i + 1]
							.letter_determined
							.entry(next_letter)
							.or_insert_with(BTreeSet::new)
							.insert(TwoActions(letter));
						if i + 3 < input.len() {
							let next_next_letter = input[i + 3];
							// ?RL
							actions[i + 1]
								.letter_determined
								.entry(next_next_letter)
								.or_insert_with(BTreeSet::new)
								.insert(ThreeActions(letter, next_letter));
						}
					}
				},
				ThreeActions(letter, next_letter) => {
					actions[i + 1]
						.letter_determined
						.entry(letter)
						.or_insert_with(BTreeSet::new)
						.insert(TwoActions(next_letter));
				},
			}
		}
	}
	result
}

pub fn main() {
	// ----------------------------- Fast IO ------------------------------ //
	let stdout = stdout();
	let mut writer = BufWriter::new(stdout.lock());
	macro_rules! print { ($($x:tt)*) => { write!(writer, $($x)*).unwrap() }; }
	macro_rules! println { ($($x:tt)*) => { writeln!(writer, $($x)*).unwrap() }; }
	#[rustfmt::skip] macro_rules! flush { ($($x:tt)*) => { writer.flush().unwrap() }; }

	let input = stdin();
	let mut scanner = Scanner::new(input.lock().bytes().map(|x| x.unwrap()));
	#[rustfmt::skip] macro_rules! read { ($t:tt) => { scanner.read::<$t>() }; }
	#[rustfmt::skip] macro_rules! readln { ($t:tt) => { scanner.readln::<$t>() }; }
	#[rustfmt::skip] macro_rules! byte { () => { scanner.byte() }; }
	#[rustfmt::skip] macro_rules! bytes { () => { scanner.bytes() }; }
	// -------------------------------------------------------------------- //

	let t = read!(usize);
	for _ in 0..t {
		let _n = read!(i64);
		let k = read!(u8);
		let a = bytes!().into_iter().map(|c| c - b'a').collect::<Vec<u8>>();
		let better = |i: u8| -> u8 {
			if i == 0 {
				return i;
			}
			if i + 1 == k {
				return 0;
			}
			i - 1
		};
		let answer = solve(&a, better)
			.into_iter()
			.map(|x| char::from(x + b'a'))
			.collect::<String>();
		println!("{}", answer);
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

    pub fn byte(&mut self) -> u8 { self.iter.next().unwrap() }

    pub fn read<T: FromStr>(&mut self) -> T
    where T::Err: Debug {
        self.buf.clear();
        let mut skip_spaces = true;
        while let Some(byte) = self.iter.peek() {
            if skip_spaces {
                if !byte.is_ascii_whitespace() {
                    skip_spaces = false;
                    self.buf.push(*byte);
                }
            } else {
                if byte.is_ascii_whitespace() {
                    break;
                } else {
                    self.buf.push(*byte);
                }
            }
            self.iter.next();
        }
        let s = std::str::from_utf8(&self.buf).unwrap_or_else(|_| panic!("input is not utf8: {:?}", self.buf));
        T::from_str(s).unwrap_or_else(|err| panic!("cannot parse from {:?}, error: {:?}", s, err))
    }

    pub fn bytes(&mut self) -> Vec<u8> {
        let mut result = Vec::new();
        while self.iter.peek().copied().map(is_ascii_newline).unwrap_or(false) {
            self.iter.next();
        }
        while let Some(byte) = self.iter.peek().copied() {
            self.iter.next();
            if is_ascii_newline(byte) {
                break;
            } else {
                result.push(byte);
            }
        }
        result
    }

    fn skip_spaces_but_not_newlines(&mut self) {
        while self.iter.peek().map(|x| x.is_ascii_whitespace() && !is_ascii_newline(*x)).unwrap_or(false) {
            self.iter.next();
        }
    }

    fn skip_one_newline(&mut self) {
        if self.iter.peek().copied().map(is_ascii_newline).unwrap_or(false) {
            self.iter.next();
        }
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
}

