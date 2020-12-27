/*****************************************************************************
* Generated and tested by: olytest    (https://github.com/optozorax/olytest) *
* Author: Ilya Sheprut                                      a.k.a. optozorax *
* Generated at:                              Mon, 28 Dec 2020 04:53:31 +0700 *
* License: MIT/Apache 2.0                                                    *
*****************************************************************************/

static mut XS: *mut Vec<i64> = 0 as _;
static mut YS: *mut Vec<i64> = 0 as _;

fn init() {
	let mut xs = Box::new(vec![0i64; 4]);
	unsafe {
		XS = &mut *xs;
	}
	std::mem::forget(xs);

	let mut ys = Box::new(vec![0i64; 4]);
	unsafe {
		YS = &mut *ys;
	}
	std::mem::forget(ys);
}

fn cost(points: &[(i64, i64)], k: i64) -> i64 {
	points
		.iter()
		.enumerate()
		.map(|(index, (x, _))| if [1, 3].contains(&index) { x - k } else { *x })
		.zip(unsafe { (*XS).iter_mut() })
		.for_each(|(value, x)| *x = value);
	points
		.iter()
		.enumerate()
		.map(|(index, (_, y))| if [2, 3].contains(&index) { y - k } else { *y })
		.zip(unsafe { (*YS).iter_mut() })
		.for_each(|(value, y)| *y = value);
	let opt_x = median(unsafe { &mut (*XS) });
	let opt_y = median(unsafe { &mut (*YS) });

	let square = [(opt_x, opt_y), (opt_x + k, opt_y), (opt_x, opt_y + k), (opt_x + k, opt_y + k)];

	points
		.iter()
		.zip(square.iter())
		.map(|((x, y), (x1, y1))| (x - x1).abs() + (y - y1).abs())
		.sum()
}

fn ternary_search<F: Fn(i64) -> i64>(mut left: i64, mut right: i64, f: F) -> i64 {
	while (right - left).abs() > 3 {
		let a = (left * 2 + right) / 3;
		let b = (left + right * 2) / 3;
		if f(a) < f(b) {
			right = b;
		} else {
			left = a;
		}
	}
	(left..=right).map(|x| (x, f(x))).min_by_key(|(_, x)| *x).unwrap().0
}

fn delta_binary_search<F: Fn(i64) -> i64>(mut a: i64, mut b: i64, f: F) -> i64 {
	while a < b {
		let c = (a + b) / 2;
		if f(c) < f(c + 1) {
			b = c;
		} else {
			a = c + 1;
		}
	}
	a
}

fn solve(mut points: &mut [(i64, i64)]) -> i64 {
	let iter = points.iter().map(|(x, y)| vec![x, y].into_iter()).flatten();
	let start = iter.clone().min().unwrap();
	let end = iter.clone().max().unwrap();
	let max_size = end - start;
	let mut min = cost(&points, 0);
	permutations(&mut points, |a| {
		min = std::cmp::min(cost(a, delta_binary_search(0, max_size, |pos| cost(a, pos))), min);
	});
	min
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

	init();
	let t = read!(usize);
	for _ in 0..t {
		let mut points = Vec::new();
		for _ in 0..4 {
			let x = read!(i64);
			let y = read!(i64);
			points.push((x, y));
		}
		println!("{}", solve(&mut points));
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

//----------------------------------------------------------------------------

// Returned value `x` which minimizes `|x0-x|+|x1-x|+...+|xn-x|`
fn median<T: Ord + Clone>(x: &mut [T]) -> T {
    assert!(!x.is_empty());
    x.sort_unstable();
    x[x.len() / 2].clone()
}

// Allocating version of function `median`
fn median_clone<T: Ord + Clone>(x: &[T]) -> T {
    let mut x = x.to_vec();
    median(&mut x)
}

//----------------------------------------------------------------------------

// Iterates over all permutation of array
// Make it iterator is impossible because of lack of GAT and therefore StreamingIterator
fn permutations<T, F: FnMut(&[T])>(a: &mut [T], mut f: F) {
    fn helper<T, F: FnMut(&[T])>(k: usize, a: &mut [T], f: &mut F) {
        if k == 1 {
            f(a);
        } else {
            helper(k - 1, a, f);
            for i in 0..k - 1 {
                if k % 2 == 0 {
                    a.swap(i, k - 1);
                } else {
                    a.swap(0, k - 1);
                }
                helper(k - 1, a, f);
            }
        }
    }
    helper(a.len(), a, &mut f);
}

