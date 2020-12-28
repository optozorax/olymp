/*****************************************************************************
* Generated and tested by: olytest    (https://github.com/optozorax/olytest) *
* Author: Ilya Sheprut                                      a.k.a. optozorax *
* Generated at:                              Tue, 29 Dec 2020 00:20:53 +0700 *
* License: MIT/Apache 2.0                                                    *
*****************************************************************************/

#[allow(clippy::mut_range_bound)]
fn factorize(mut n: u64) -> BTreeMap<u64, usize> {
	let mut result = BTreeMap::new();
	for i in 2..n {
		if i > n || i > (n as f64).sqrt() as u64 + 5 {
			break;
		}
		while n % i == 0 {
			*result.entry(i).or_insert(0) += 1;
			n /= i;
		}
	}
	if n != 0 {
		*result.entry(n).or_insert(0) += 1;
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
		let mut n = read!(u64);
		let max_count = factorize(n)
			.into_iter()
			.max_by_key(|(_miltiprier, count)| *count)
			.unwrap();
		println!("{}", max_count.1);
		for _ in 0..max_count.1 - 1 {
			print!("{} ", max_count.0);
			n /= max_count.0;
		}
		println!("{}", n);
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

