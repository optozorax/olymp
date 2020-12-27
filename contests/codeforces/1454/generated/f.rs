/*****************************************************************************
* Generated and tested by: olytest    (https://github.com/optozorax/olytest) *
* Author: Ilya Sheprut                                      a.k.a. optozorax *
* Generated at:                              Mon, 28 Dec 2020 04:16:33 +0700 *
* License: MIT/Apache 2.0                                                    *
*****************************************************************************/

fn solve(arr: &[u32]) -> Option<(usize, usize)> {
	let mut z_maxs = Vec::with_capacity(arr.len());
	let mut current_max = *arr.last().unwrap();
	for i in arr.iter().rev().skip(1).take(arr.len() - 2) {
		z_maxs.push(current_max);
		current_max = std::cmp::max(current_max, *i);
	}

	let min_tree = Rmq::create(arr, RmqType::Min);
	let mut current_max = arr[0];
	for (x_size, i) in arr.iter().enumerate().skip(1).take(arr.len() - 2) {
		if let Some(min_range) = binary_search_number_range(x_size..arr.len() - 1, current_max, |pos| {
			min_tree.most_for_segment(x_size..pos + 1)
		}) {
			if let Some(final_y_pos) =
				binary_search_number_range(min_range, current_max, |pos| z_maxs[arr.len() - pos - 2])
			{
				let y_size = final_y_pos.start - x_size + 1;
				return Some((x_size, y_size));
			}
		}
		current_max = std::cmp::max(current_max, *i);
	}

	None
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

	let t = read!(usize);
	for _ in 0..t {
		let n = read!(usize);
		let vec = readln!(u32);
		if let Some((x, y)) = solve(&vec) {
			println!("YES");
			println!("{} {} {}", x, y, n - (x + y));
		} else {
			println!("NO");
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

// It is supposed that `f` returns `false` `n` times, then `true` `n` times. This method uses binary search to find `n`. This function calls `f` only on range [on.start; on.end).
fn binary_search<F: Fn(usize) -> bool>(on: Range<usize>, f: F) -> Option<usize> {
	let mut a = on.start;
	let mut b = on.end;
	match b.checked_sub(a)? {
		0 => None,
		1 => Some(a).filter(|&x| f(x)),
		_ => if f(a) {
			Some(a)
		} else if !f(b-1) {
			None
		} else {
			b -= 1;
			loop {
				let c = a+(b-a)/2;
				match b-a {
					0 => unreachable!(),
					1 => return Some(b),
					2 => return Some(if f(c) { c } else { b }),
					_ => if f(c) { b = c; } else { a = c; }
				}
			}
		}
	}
}

//----------------------------------------------------------------------------

// Finds leftmost position in `on` where number are equals to `number`. `f` must be monotone function.
fn binary_search_number_leftmost<F: Fn(usize) -> T, T: Ord + Eq + Copy + Debug>(on: Range<usize>, number: T, f: F) -> Option<usize> {
	match on.end.checked_sub(on.start)? {
		0 => None,
		1 => Some(on.start).filter(|&x| f(x) == number),
		_ => {
			let left = f(on.start);
			let right = f(on.end-1);
			if left > right {
				binary_search(on, |pos| f(pos) <= number).filter(|x| f(*x) == number)
			} else if left < right {
				binary_search(on, |pos| f(pos) >= number).filter(|x| f(*x) == number)
			} else if left == number {
				Some(on.start)
			} else {
				None
			}
		}
	}
}

//----------------------------------------------------------------------------

// Finds range in `on` where all numbers are equals to `number`. `f` must be monotone function.
fn binary_search_number_range<F: Fn(usize) -> T, T: Ord + Eq + Copy + Debug>(on: Range<usize>, number: T, f: F) -> Option<Range<usize>> {
	let left = binary_search_number_leftmost(on.clone(), number, &f)?;
	let right = binary_search(left..on.end, |pos| f(pos) != number).unwrap_or(on.end);
	Some(left..right)
}

//----------------------------------------------------------------------------

trait BinaryFunction {
	type T: Clone;
	type Y: Clone;

	fn up(&self, pos: usize, t: Self::T) -> Self::Y;
	fn f(&self, a: Self::Y, b: Self::Y) -> Self::Y;
}

impl<T, Y, F1, F2> BinaryFunction for (F1, F2, PhantomData<T>, PhantomData<Y>)
where
	T: Clone,
	Y: Clone,
	F1: Fn(usize, T) -> Y,
	F2: Fn(Y, Y) -> Y,
{
	type T = T;
	type Y = Y;

	fn up(&self, pos: usize, t: Self::T) -> Self::Y { (self.0)(pos, t) }

	fn f(&self, a: Self::Y, b: Self::Y) -> Self::Y { (self.1)(a, b) }
}

//----------------------------------------------------------------------------

/// https://cp-algorithms.com/data_structures/sparse-table.html
/// Can be applied to all idempotent function. Idempotent function is f(x, x) = x. Examples: gcd, min, max.
#[derive(Debug)]
struct SparseTable<'a, B: BinaryFunction> {
	input: &'a [B::T],
	inner: Vec<Vec<B::Y>>,
	b: B,
}

impl<'a, B: BinaryFunction> SparseTable<'a, B> {
	pub fn create(input: &'a [B::T], b: B) -> Self {
		let levels = Self::largest_bit(input.len());
		let mut inner = Vec::with_capacity(levels);
		inner.push(
			(0..input.len())
				.map(|pos| b.up(pos, input[pos].clone()))
				.collect::<Vec<B::Y>>(),
		);
		let mut result = Self { input, inner, b };
		for k in 1..=levels {
			let previous = result.inner.last().unwrap();
			let to_push = (0..(input.len() - (1 << k) + 1))
				.map(|i| result.b.f(previous[i].clone(), previous[i + (1 << (k - 1))].clone()))
				.collect::<Vec<_>>();
			result.inner.push(to_push);
		}
		result
	}

	fn largest_bit(a: usize) -> usize {
		(std::mem::size_of::<usize>() * 8 - 1).saturating_sub(a.leading_zeros() as usize)
	}

	pub fn f_for_segment(&self, on: Range<usize>) -> B::Y {
		assert!(on.start < on.end);
		let a = on.start;
		let b = on.end;
		let level = Self::largest_bit(b - a);
		self.b
			.f(self.inner[level][a].clone(), self.inner[level][b - (1 << level)].clone())
	}
}

//----------------------------------------------------------------------------

#[derive(Debug)]
struct MinOrMax<T> {
	kind: RmqType,
	phantom: PhantomData<T>,
}

impl<T: Clone + Ord> BinaryFunction for MinOrMax<T> {
	type T = T;
	type Y = (usize, T);

	fn up(&self, pos: usize, t: Self::T) -> Self::Y { (pos, t) }

	fn f(&self, a: Self::Y, b: Self::Y) -> Self::Y {
		match self.kind {
			RmqType::Min => {
				if a.1 < b.1 {
					a
				} else {
					b
				}
			},
			RmqType::Max => {
				if a.1 > b.1 {
					a
				} else {
					b
				}
			},
		}
	}
}

#[derive(Debug)]
struct Rmq<'a, T: Clone + Ord> {
	sparse_table: SparseTable<'a, MinOrMax<T>>,
}

#[derive(Clone, Debug)]
enum RmqType {
	Min,
	Max,
}

impl<'a, T: Ord + Copy> Rmq<'a, T> {
	pub fn create(input: &'a [T], kind: RmqType) -> Self {
		let binary_function = MinOrMax { kind, phantom: PhantomData };
		let sparse_table = SparseTable::create(input, binary_function);
		Self { sparse_table }
	}

	pub fn most_index_for_segment(&self, on: Range<usize>) -> usize { self.sparse_table.f_for_segment(on).0 }

	pub fn most_for_segment(&self, on: Range<usize>) -> T {
		assert!(on.start != on.end);
		self.sparse_table.f_for_segment(on).1
	}
}

