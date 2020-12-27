/*****************************************************************************
 * Generated and tested by: olytest   (https://github.com/optozorax/olytest) *
 * Author: Ilya Sheprut                                     a.k.a. optozorax *
 * Generated at:                             Mon, 28 Dec 2020 00:52:46 +0700 *
 * License: MIT/Apache 2.0                                                   *
 *****************************************************************************
 */

#![allow(unused_imports, clippy::many_single_char_names)]
use std::{
	collections::BTreeMap,
	convert::{TryFrom, TryInto},
	io::{BufRead, Read, Write},
	iter::{FromIterator, Peekable},
	str::FromStr,
};

struct MyMap<T>(T);
struct Input(Peekable<MyMap<std::io::Bytes<std::io::Stdin>>>);

impl<I: Iterator<Item = Result<u8, std::io::Error>>> Iterator for MyMap<I> {
	type Item = u8;

	#[inline]
	fn next(&mut self) -> Option<u8> { self.0.next().map(|x| x.unwrap()) }
}

fn get_input() -> &'static mut Input {
	static mut SINGLETON: *mut Input = 0 as *mut Input;
	static ONCE: Once = Once::new();

	use std::{mem, sync::Once};

	unsafe {
		ONCE.call_once(|| {
			let singleton: Input = Input(MyMap(std::io::stdin().bytes()).peekable());
			SINGLETON = mem::transmute(Box::new(singleton));
		});
		&mut *SINGLETON
	}
}

macro_rules! on_num_type {
	(u8 $on_unum:tt $on_inum:tt $on_fnum:tt) => {
		$on_unum
	};
	(u16 $on_unum:tt $on_inum:tt $on_fnum:tt) => {
		$on_unum
	};
	(u32 $on_unum:tt $on_inum:tt $on_fnum:tt) => {
		$on_unum
	};
	(u64 $on_unum:tt $on_inum:tt $on_fnum:tt) => {
		$on_unum
	};
	(u128 $on_unum:tt $on_inum:tt $on_fnum:tt) => {
		$on_unum
	};
	(usize $on_unum:tt $on_inum:tt $on_fnum:tt) => {
		$on_unum
	};

	(i8 $on_unum:tt $on_inum:tt $on_fnum:tt) => {
		$on_inum
	};
	(i16 $on_unum:tt $on_inum:tt $on_fnum:tt) => {
		$on_inum
	};
	(i32 $on_unum:tt $on_inum:tt $on_fnum:tt) => {
		$on_inum
	};
	(i64 $on_unum:tt $on_inum:tt $on_fnum:tt) => {
		$on_inum
	};
	(i128 $on_unum:tt $on_inum:tt $on_fnum:tt) => {
		$on_inum
	};
	(isize $on_unum:tt $on_inum:tt $on_fnum:tt) => {
		$on_inum
	};

	(f32 $on_unum:tt $on_inum:tt $on_fnum:tt) => {
		$on_fnum
	};
	(f64 $on_unum:tt $on_inum:tt $on_fnum:tt) => {
		$on_fnum
	};
}

macro_rules! read {
	([$($inside:tt)*]) => {{ read!(@init input); read!(@read input [$($inside)*]) }};
	($inside:ident) => {{ read!(@init input); read!(@read input $inside) }};
	(_) => {{ read!(@init input); read!(@spaces input) }};

	(@read $input:ident [$type:tt]) => {{
		let mut result = Vec::new();
		while $input.peek().as_ref().map(|&&b| b != b'\r' && b != b'\n').unwrap_or(false) {
			result.push(read!(@read $input $type));
		}
		result
	}};

	(@read $input:ident [$type:tt; $count:tt]) => {{
		let mut result = Vec::new();
		for _ in 0..$count {
			result.push(read!(@read $input $type));
			read!(@newlines $input);
		}
		result
	}};

	(@read $input:ident SpacesString) => { read!(@string $input) };
	(@read $input:ident String) => {{ read!(@spaces $input); read!(@string $input) }};
	(@read $input:ident char) => {{ read!(@spaces $input); read!(@char $input) }};
	(@read $input:ident u8char) => {{ read!(@spaces $input); read!(@u8char $input) }};
	(@read $input:ident u8digit) => {{ read!(@spaces $input); read!(@u8digit $input) }};
	(@read $input:ident u8lowletter) => {{ read!(@spaces $input); read!(@u8lowletter $input) }};
	(@read $input:ident u8upletter) => {{ read!(@spaces $input); read!(@u8upletter $input) }};

	(@read $input:ident $type:ident) => {
		on_num_type!($type {
			read!(@unum $input $type)
		} {
			read!(@inum $input $type)
		} {
			read!(@fnum $input $type)
		})
	};

	(@init $input:ident) => {
		let $input = &mut get_input().0;
	};

	(@unum $input:ident $usize:ident) => {{
		let mut s = String::new();
		read!(@spaces $input);
		read!(@numbers $input s);
		if s.is_empty() { panic!("can't read number"); }
		s.parse::<$usize>().unwrap()
	}};

	(@inum $input:ident $isize:ident) => {{
		let mut s = String::new();
		read!(@spaces $input);
		read!(@maybechar $input b'-' s {});
		read!(@numbers $input s);
		if s.is_empty() { panic!("can't read number"); }
		s.parse::<$isize>().unwrap()
	}};

	(@fnum $input:ident $fsize:ident) => {{
		let mut s = String::new();
		read!(@spaces $input);
		read!(@maybechar $input b'-' s {});
		read!(@numbers $input s);
		read!(@maybechar $input b'.' s {
			read!(@numbers $input s);
		});
		if s.is_empty() { panic!("can't read number"); }
		s.parse::<$fsize>().unwrap()
	}};

	(@maybechar $input:ident $char:literal $to_push_string:ident $if_present:expr) => {{
		if $input.peek().as_ref().map(|x| **x == $char).unwrap_or(false) {
			$to_push_string.push($input.next().unwrap().into());
			$if_present;
		}
	}};

	(@numbers $input:ident $to_push_string:ident) => {{
		loop {
			if $input.peek().as_ref().map(|x| x.is_ascii_digit()).unwrap_or(false) {
				$to_push_string.push($input.next().unwrap().into());
			} else {
				break;
			}
		}
	}};

	(@spaces $input:ident) => {{
		while $input
			.peek()
			.as_ref()
			.map(|&&b| b == b' ' || b == b'\n' || b == b'\r' || b == b'\t')
			.unwrap_or(false)
		{
			$input.next().unwrap();
		}
	}};

	(@newlines $input:ident) => {{
		while $input
			.peek()
			.as_ref()
			.map(|&&b| b == b'\n' || b == b'\r')
			.unwrap_or(false)
		{
			$input.next().unwrap();
		}
	}};

	(@string $input:ident) => {{
		let mut v = Vec::new();
		while $input.peek().as_ref().map(|&&b| b != b'\r' && b != b'\n').unwrap_or(false) {
			v.push($input.next().unwrap());
		}
		String::from_utf8(v).unwrap()
	}};

	(@char $input:ident) => {{
		char::from($input.next().unwrap())
	}};

	(@u8char $input:ident) => {{
		$input.next().unwrap()
	}};

	(@u8digit $input:ident) => {{
		$input.next().unwrap() - b'0'
	}};

	(@u8lowletter $input:ident) => {{
		$input.next().unwrap() - b'a'
	}};

	(@u8upletter $input:ident) => {{
		$input.next().unwrap() - b'A'
	}};
}

trait FindDuplicatesWithVecBy: Sized {
	fn duplicates_with_vec_by<T: Clone, F: Fn(T) -> usize>(self, vec_size: usize, f: F) -> DuplicatesWithVecBy<Self, F, T> where Self: std::iter::Iterator<Item = T>;
}

impl<I: Iterator> FindDuplicatesWithVecBy for I {
	fn duplicates_with_vec_by<T: Clone, F: Fn(T) -> usize>(self, vec_size: usize, f: F) -> DuplicatesWithVecBy<Self, F, T> where Self: std::iter::Iterator<Item = T> {
		DuplicatesWithVecBy {
			vec: vec![None; vec_size],
			iter: self,
			f,
		}
	}
}

struct DuplicatesWithVecBy<I: Iterator<Item = T>, F: Fn(T) -> usize, T: Clone> {
	vec: Vec<Option<T>>,
	iter: I,
	f: F,
}

impl<I: Iterator<Item = T>, F: Fn(T) -> usize, T: Clone> Iterator for DuplicatesWithVecBy<I, F, T> {
	type Item = (T, T);

	fn next(&mut self) -> Option<Self::Item> {
		while let Some(elem) = self.iter.next() {
			let pos = (self.f)(elem.clone());
			dbg!(pos);
			if let Some(exists) = &self.vec[pos] {
				return Some((exists.clone(), elem))
			} else {
				self.vec[pos] = Some(elem);
			}
		}
		None
	}
}


enum Get {
	Xor,
	And,
}

fn get(kind: Get, i: usize, j: usize) -> usize {
	let s = match kind {
		Get::Xor => "XOR",
		Get::And => "AND",
	};
	println!("{} {} {}", s, i + 1, j + 1);
	std::io::stdout().flush().unwrap();
	read!(usize)
}

pub fn main() {
	let count = read!(usize);

	let xored = {
		let mut result: Vec<usize> = Vec::new();
		for i in 1..count {
			result.push(get(Get::Xor, 0, i))
		}
		result
	};

	let first = if let Some(zero_pos) = xored.iter().position(|x| *x == 0) {
		// there is same value as first
		xored[zero_pos] ^ get(Get::And, 0, zero_pos + 1)
	} else if let Some((a, b)) = xored
		.iter()
		.enumerate()
		.duplicates_with_vec_by(count, |(_, value)| *value)
		.map(|(a, b)| (a.0, b.0))
		.next()
	{
		// there is two same values
		xored[a] ^ get(Get::And, a + 1, b + 1)
	} else {
		// there is all different numbers
		let a = get(Get::And, 0, xored.iter().position(|x| *x == 1).unwrap() + 1);
		let b = get(Get::And, 0, xored.iter().position(|x| *x == 2).unwrap() + 1);
		a | b
	};

	print!("! {}", first);
	for i in xored {
		print!(" {}", i ^ first);
	}
	println!();
	std::io::stdout().flush().unwrap();
}
