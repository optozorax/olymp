/*****************************************************************************
* Generated and tested by: olytest    (https://github.com/optozorax/olytest) *
* Author: Ilya Sheprut                                      a.k.a. optozorax *
* Generated at:                              Tue, 29 Dec 2020 00:20:35 +0700 *
* License: MIT/Apache 2.0                                                    *
*****************************************************************************/

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
	// -------------------------------------------------------------------- //

	let count = read!(usize);

	enum Get {
		Xor,
		And,
	}

	let mut get = |kind: Get, i: usize, j: usize| -> usize {
		let s = match kind {
			Get::Xor => "XOR",
			Get::And => "AND",
		};
		println!("{} {} {}", s, i + 1, j + 1);
		flush!();
		read!(usize)
	};

	let xored = {
		let mut result: Vec<usize> = Vec::new();
		for i in 1..count {
			result.push(get(Get::Xor, 0, i))
		}
		result
	};

	let first = if let Some(zero_pos) = xored.iter().position(|x| *x == 0) {
		// there is same value as first
		xored[zero_pos] ^ get(Get::And, 0, zero_pos + 1)
	} else if let Some((a, b)) = xored
		.iter()
		.enumerate()
		.duplicates_with_vec_by(count, |(_, value)| *value)
		.map(|(a, b)| (a.0, b.0))
		.next()
	{
		// there is two same values
		xored[a] ^ get(Get::And, a + 1, b + 1)
	} else {
		// there is all different numbers
		let a = get(Get::And, 0, xored.iter().position(|x| *x == 1).unwrap() + 1);
		let b = get(Get::And, 0, xored.iter().position(|x| *x == 2).unwrap() + 1);
		a | b
	};

	print!("! {}", first);
	for i in xored {
		print!(" {}", i ^ first);
	}
	println!();
	flush!();
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

//----------------------------------------------------------------------------

trait FindDuplicatesWithVecBy: Sized {
	fn duplicates_with_vec_by<T: Clone, F: Fn(T) -> usize>(self, vec_size: usize, f: F) -> DuplicatesWithVecBy<Self, F, T> where Self: std::iter::Iterator<Item = T>;
}

impl<I: Iterator> FindDuplicatesWithVecBy for I {
	fn duplicates_with_vec_by<T: Clone, F: Fn(T) -> usize>(self, vec_size: usize, f: F) -> DuplicatesWithVecBy<Self, F, T> where Self: std::iter::Iterator<Item = T> {
		DuplicatesWithVecBy {
			vec: vec![None; vec_size],
			iter: self,
			f,
		}
	}
}

struct DuplicatesWithVecBy<I: Iterator<Item = T>, F: Fn(T) -> usize, T: Clone> {
	vec: Vec<Option<T>>,
	iter: I,
	f: F,
}

impl<I: Iterator<Item = T>, F: Fn(T) -> usize, T: Clone> Iterator for DuplicatesWithVecBy<I, F, T> {
	type Item = (T, T);

	fn next(&mut self) -> Option<Self::Item> {
		while let Some(elem) = self.iter.next() {
			let pos = (self.f)(elem.clone());
			dbg!(pos);
			if let Some(exists) = &self.vec[pos] {
				return Some((exists.clone(), elem))
			} else {
				self.vec[pos] = Some(elem);
			}
		}
		None
	}
}

