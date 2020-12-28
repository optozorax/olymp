/*****************************************************************************
* Generated and tested by: olytest    (https://github.com/optozorax/olytest) *
* Author: Ilya Sheprut                                      a.k.a. optozorax *
* Generated at:                              Mon, 28 Dec 2020 22:26:53 +0700 *
* License: MIT/Apache 2.0                                                    *
*****************************************************************************/

fn solve(h: &[i64], k: i64) -> bool {
	let mut current_range = (h[0], h[0]);
	for h in h.iter().copied().skip(1).take(h.len() - 2) {
		let h_range = (h, h + (k - 1));
		current_range = (max(current_range.0 - (k - 1), h_range.0), min(current_range.1 + (k - 1), h_range.1));
		if current_range.0 > current_range.1 {
			return false;
		}
	}
	let last = *h.last().unwrap();
	current_range.0 - (k - 1) <= last && last <= current_range.1 + (k - 1)
}

pub fn main() {
	// ----------------------------- Fast IO ------------------------------ //
	let stdout = stdout();
	let mut writer = BufWriter::new(stdout.lock());
	macro_rules! print { ($($x:tt)*) => { write!(writer, $($x)*).unwrap() }; }
	macro_rules! println { ($($x:tt)*) => { writeln!(writer, $($x)*).unwrap() }; }
	#[rustfmt::skip] macro_rules! flush { ($($x:tt)*) => { writer.flush().unwrap() }; }

	let input = std::io::stdin();
	let mut stdin = input.lock().lines();
	#[rustfmt::skip] macro_rules! read { () => { read(&mut stdin) }; }
	// -------------------------------------------------------------------- //

	let t: usize = read!();
	for _ in 0..t {
		let SpaceTuple2(_n, k): SpaceTuple2<i64, i64> = read!();
		let SpaceVec(h): SpaceVec<i64> = read!();
		if solve(&h, k) {
			println!("YES");
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


fn read<T: FromStr, I: Iterator<Item = std::io::Result<String>>>(i: &mut I) -> T
where <T as std::str::FromStr>::Err: std::fmt::Debug {
	i.next().unwrap().unwrap().parse().unwrap()
}

struct SpaceVec<T>(pub Vec<T>);
impl<T: FromStr> FromStr for SpaceVec<T>
where <T as FromStr>::Err: std::error::Error + 'static
{
	type Err = Box<dyn std::error::Error>;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(SpaceVec(
			s.split_whitespace()
				.map(|x| x.parse::<T>())
				.collect::<Result<Vec<_>, _>>()?,
		))
	}
}

// Allows to read two different types, separated by space
struct SpaceTuple2<A, B>(pub A, pub B);
impl<A: FromStr, B: FromStr> FromStr for SpaceTuple2<A, B>
where
	<A as FromStr>::Err: std::error::Error + 'static,
	<B as FromStr>::Err: std::error::Error + 'static,
{
	type Err = Box<dyn std::error::Error>;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut iter = s.split_whitespace();
		let a = A::from_str(iter.next().ok_or_else(|| Box::new(Error))?)?;
		let b = B::from_str(iter.next().ok_or_else(|| Box::new(Error))?)?;
		Ok(SpaceTuple2(a, b))
	}
}
