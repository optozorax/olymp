/*****************************************************************************
* Generated and tested by: olytest    (https://github.com/optozorax/olytest) *
* Author: Ilya Sheprut                                      a.k.a. optozorax *
* Generated at:                              Tue, 29 Dec 2020 00:21:02 +0700 *
* License: MIT/Apache 2.0                                                    *
*****************************************************************************/

fn solve(x: i64) -> i64 {
	let a = ArithmeticProgresion::new(1, 1);
	let pos = a.from_sum(x);
	let sum = a.sum_to(pos);

	let sum1 = a.sum_to(pos + 1);
	if x == sum {
		pos
	} else if x == sum1 - 1 {
		pos + 2
	} else {
		pos + 1
	}
}

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
		let x = read!(i64);
		println!("{}", solve(x));
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/ \/ TESTING CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
//----------------------------------------------------------------------------

fn solve_brute_force(x: i64, k: i64, y: i64) -> usize {
	if y == x {
		return 0;
	}
	if k > 20 {
		return 100;
	}
	1 + min(solve_brute_force(x, k + 1, y + k), solve_brute_force(x, k + 1, y - 1))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn name() {
		for i in 0..40 {
			eprintln!("{}: {}, {}", i, solve_brute_force(i, 1, 0), solve(i));
			assert_eq!(solve_brute_force(i, 1, 0) as i64, solve(i));
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

#[derive(Debug, Clone)]
pub enum QuadraticSolvingResult {
	Two(f64, f64),
	One(f64),
	Zero,
}

pub fn solve_quadratic_equation(a: f64, b: f64, c: f64) -> QuadraticSolvingResult {
	use QuadraticSolvingResult::*;
	let d = b*b - 4.0*a*c;
	if d < 0.0 {
		Zero
	} else if d.abs() < 1e-9 {
		One(-b/(2.0*a))
	} else {
		let sq_d = d.sqrt();
		let x1 = (-b + sq_d)/(2.0*a);
		let x2 = (-b - sq_d)/(2.0*a);
		Two(
			x1.max(x2),
			x1.min(x2),
		)
	}
}

//----------------------------------------------------------------------------

pub struct ArithmeticProgresion {
    a1: i64,
    d: i64,
}

impl ArithmeticProgresion {
    pub fn new_canonical() -> Self {
        Self {
            a1: 1,
            d: 1,
        }
    }

    pub fn new(a1: i64, d: i64) -> Self {
        Self {
            a1,
            d,
        }
    }

    /// N-th elemnt of arithmetic progression
    pub fn nth(&self, n: i64) -> i64 {
        self.a1 + self.d * n
    }

    /// Same as `(0..n).map(|x| self.nth(x)).sum()`, but `O(1)` complexity, optimized by formula.
    pub fn sum_to(&self, n: i64) -> i64 {
        (2*self.a1 + self.d*(n-1))*n/2
    }

    /// For this sum returns such `n` that: `self.sum_to(n) <= sum < self.sum_to(n+1)`.
    pub fn from_sum(&self, sum: i64) -> i64 {
        use QuadraticSolvingResult::*;
        let a = self.d as f64;
        let b = 2.0*self.a1 as f64 - self.d as f64;
        let c = -2.0 * sum as f64;
        match solve_quadratic_equation(a, b, c) {
            Two(a, _) => a as i64, // TODO think about another?
            One(a) => a as i64,
            Zero => 0,
        }
    }
}

