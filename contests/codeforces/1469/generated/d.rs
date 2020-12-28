/*****************************************************************************
* Generated and tested by: olytest    (https://github.com/optozorax/olytest) *
* Author: Ilya Sheprut                                      a.k.a. optozorax *
* Generated at:                              Tue, 29 Dec 2020 00:20:24 +0700 *
* License: MIT/Apache 2.0                                                    *
*****************************************************************************/

pub fn main() {
	// ----------------------------- Fast IO ------------------------------ //
	let stdout = stdout();
	let mut writer = BufWriter::new(stdout.lock());
	macro_rules! println { ($($x:tt)*) => { writeln!(writer, $($x)*).unwrap() }; }

	let input = stdin();
	let mut scanner = Scanner::new(input.lock().bytes().map(|x| x.unwrap()));
	#[rustfmt::skip] macro_rules! read { ($t:tt) => { scanner.read::<$t>() }; }
	// -------------------------------------------------------------------- //

	let t = read!(usize);
	for _ in 0..t {
		let n = read!(usize);
		if n == 3 {
			println!("2\n3 2\n3 2");
		} else if n == 4 {
			println!("3\n3 4\n4 2\n4 2");
		} else {
			// n >= 5
			let mut ops = Vec::new();
			ops.push((3, n));
			for i in 5..=min(15, n - 1) {
				ops.push((i, n));
			}
			for i in min(17, n)..n {
				ops.push((i, n));
			}
			let divisor = if n > 16 { 16 } else { 4 };
			let mut last = n;
			while last != 1 {
				ops.push((n, divisor));
				last = ceil_div(last, divisor);
			}
			if divisor == 16 {
				ops.push((16, 4));
				ops.push((16, 4));
			}
			ops.push((4, 2));
			ops.push((4, 2));

			println!("{}", ops.len());
			for (x, y) in ops {
				println!("{} {}", x, y);
			}
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

// Division with rounding up. ceil_div(2, 3) = 1, ceil_div(5, 2) = 3
fn ceil_div<T>(x: T, y: T) -> T where T: Copy + Rem<Output = T> + Div<Output = T> + Add<Output = T> + From<u8> + PartialEq {
	x / y + if x % y == T::from(0u8) { T::from(0) } else { T::from(1) }
}

