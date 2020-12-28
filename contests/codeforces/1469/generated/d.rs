/*****************************************************************************
* Generated and tested by: olytest    (https://github.com/optozorax/olytest) *
* Author: Ilya Sheprut                                      a.k.a. optozorax *
* Generated at:                              Mon, 28 Dec 2020 23:29:05 +0700 *
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
	#[rustfmt::skip] macro_rules! read { () => { read(&mut stdin) }; }
	// -------------------------------------------------------------------- //

	let t: usize = read!();
	for _ in 0..t {
		let n: usize = read!();
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
				last = last / divisor + if last % divisor == 0 { 0 } else { 1 };
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


fn read<T: FromStr, I: Iterator<Item = std::io::Result<String>>>(i: &mut I) -> T
where <T as std::str::FromStr>::Err: std::fmt::Debug {
	i.next().unwrap().unwrap().parse().unwrap()
}
