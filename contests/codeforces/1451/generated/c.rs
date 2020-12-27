/*****************************************************************************
* Generated and tested by: olytest    (https://github.com/optozorax/olytest) *
* Author: Ilya Sheprut                                      a.k.a. optozorax *
* Generated at:                              Mon, 28 Dec 2020 01:53:38 +0700 *
* License: MIT/Apache 2.0                                                    *
*****************************************************************************/

fn solve(a: &[u8], b: &[u8], k: usize) -> bool {
	let mut have = a.iter().copied().map(|x| x - b'a').collect_count(27);
	let need = b.iter().copied().map(|x| x - b'a').collect_count(27);

	for i in 0..26 {
		if (have[i] < need[i]) || ((have[i] - need[i]) % k != 0) {
			return false;
		}
		have[i] -= need[i];
		have[i + 1] += have[i];
	}

	true
}

pub fn main() {
	// ----------------------------- Fast IO ------------------------------ //
	let stdout = stdout();
	let mut writer = BufWriter::new(stdout.lock());
	macro_rules! println { ($($x:tt)*) => { writeln!(writer, $($x)*).unwrap() }; }

	let input = stdin();
	let mut scanner = Scanner::new(input.lock().bytes().map(|x| x.unwrap()));
	#[rustfmt::skip] macro_rules! read { ($t:tt) => { scanner.read::<$t>() }; }
	#[rustfmt::skip] macro_rules! bytes { () => { scanner.bytes() }; }
	// -------------------------------------------------------------------- //

	let t = read!(u32);
	for _ in 0..t {
		let _n = read!(usize);
		let k = read!(usize);
		let a = bytes!();
		let b = bytes!();
		if solve(&a, &b, k) {
			println!("Yes");
		} else {
			println!("No");
		}
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

    pub fn readln<T: FromStr>(&mut self) -> Vec<T>
    where T::Err: Debug {
        let mut result = Vec::new();
        while self.iter.peek().map(|x| !is_ascii_newline(*x)).unwrap_or(false) {
            result.push(self.read());
        }
        self.iter.next();
        result
    }
}

//----------------------------------------------------------------------------

trait CountVecTrait {
	fn collect_count(self, max: usize) -> Vec<usize>;
}

impl<T: Iterator<Item = K>, K: Into<usize>> CountVecTrait for T {
	fn collect_count(self, max: usize) -> Vec<usize> {
		let mut result = vec![0; max];
		for i in self {
			result[i.into()] += 1;
		}
		result
	}	
}

