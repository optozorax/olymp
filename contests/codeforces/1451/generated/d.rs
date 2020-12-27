/*****************************************************************************
 * Generated and tested by: olytest   (https://github.com/optozorax/olytest) *
 * Author: Ilya Sheprut                                     a.k.a. optozorax *
 * Generated at:                             Mon, 28 Dec 2020 01:23:16 +0700 *
 * License: MIT/Apache 2.0                                                   *
 *****************************************************************************
 */

fn moves_up(token: (u64, u64), d: u64, k: u64) -> Option<u64> {
	let result = ((d as f64 * d as f64 - token.0 as f64 * token.0 as f64).sqrt().floor() as u64 - token.1) / k;
	if result == 0 { None } else { Some(result) }
}

fn moves_right(token: (u64, u64), d: u64, k: u64) -> Option<u64> {
	let result = ((d as f64 * d as f64 - token.1 as f64 * token.1 as f64).sqrt().floor() as u64 - token.0) / k;
	if result == 0 { None } else { Some(result) }
}

fn is_utkarsh_wins(d: u64, k: u64) -> bool {
	let mut token = (0, 0);
	loop {
		// Ashish
		match (moves_up(token, d, k), moves_right(token, d, k)) {
			(Some(up), Some(right)) => {
				if up > right {
					token.1 += k
				} else {
					token.0 += k
				}
			},
			(Some(_), None) => token.1 += k,
			(None, Some(_)) => token.0 += k,
			(None, None) => return true,
		}

		// Utkarish
		match (moves_up(token, d, k), moves_right(token, d, k)) {
			(Some(up), Some(right)) => {
				if up > right {
					token.1 += k
				} else {
					token.0 += k
				}
			},
			(Some(_), None) => token.1 += k,
			(None, Some(_)) => token.0 += k,
			(None, None) => return false,
		}
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

	let count: i64 = read!(i64);
	for _ in 0..count {
		let d = read!(u64);
		let k = read!(u64);
		if is_utkarsh_wins(d, k) {
			println!("Utkarsh");
		} else {
			println!("Ashish");
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

