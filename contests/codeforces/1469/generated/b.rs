/*****************************************************************************
* Generated and tested by: olytest    (https://github.com/optozorax/olytest) *
* Author: Ilya Sheprut                                      a.k.a. optozorax *
* Generated at:                              Mon, 28 Dec 2020 22:04:41 +0700 *
* License: MIT/Apache 2.0                                                    *
*****************************************************************************/

pub fn main() {
	// ----------------------------- Fast IO ------------------------------ //
	let stdout = stdout();
	let mut writer = BufWriter::new(stdout.lock());
	macro_rules! print { ($($x:tt)*) => { write!(writer, $($x)*).unwrap() }; }
	macro_rules! println { ($($x:tt)*) => { writeln!(writer, $($x)*).unwrap() }; }
	#[rustfmt::skip] macro_rules! flush { ($($x:tt)*) => { writer.flush().unwrap() }; }

	let input = std::io::stdin();
	let mut stdin = input.lock().lines();
	#[rustfmt::skip] macro_rules! read { ($($x:tt)*) => { read(&mut stdin) }; }
	// -------------------------------------------------------------------- //

	let t: usize = read!();
	for _ in 0..t {
		let _n: usize = read!();
		let SpaceVec(mut r): SpaceVec<i32> = read!();
		let _m: usize = read!();
		let SpaceVec(mut b): SpaceVec<i32> = read!();
		r.push(0);
		b.push(0);

		let mut sum = 0;
		let max1 = r
			.iter()
			.map(|x| {
				let ans = sum;
				sum += x;
				ans
			})
			.max()
			.unwrap();

		let mut sum = 0;
		let max2 = b
			.iter()
			.map(|x| {
				let ans = sum;
				sum += x;
				ans
			})
			.max()
			.unwrap();

		println!("{}", max1 + max2);
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
