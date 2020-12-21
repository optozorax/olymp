#![allow(unused_imports, clippy::many_single_char_names)]
use std::{
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
}

fn solve(s: &[char], l: usize, r: usize) -> bool {
	let first_pos = s.iter().position(|x| *x == s[l]).unwrap();
	let last_pos = s.len() - 1 - s.iter().rev().position(|x| *x == s[r]).unwrap();
	first_pos != l || last_pos != r
}

pub fn main() {
	let t = read!(u8);
	for _ in 0..t {
		let n = read!(u8);
		let q = read!(u8);
		let s = read!([char; n]);
		for _ in 0..q {
			let l = read!(usize) - 1;
			let r = read!(usize) - 1;
			if solve(&s, l, r) {
				println!("YES");
			} else {
				println!("NO");
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn name() {
		assert_eq!(solve(&"001000".chars().collect::<Vec<char>>(), 2 - 1, 4 - 1), true);
		assert_eq!(solve(&"001000".chars().collect::<Vec<char>>(), 0, 3 - 1), false);
		assert_eq!(solve(&"001000".chars().collect::<Vec<char>>(), 3 - 1, 5 - 1), true);

		assert_eq!(solve(&"1111".chars().collect::<Vec<char>>(), 0, 4 - 1), false);
		assert_eq!(solve(&"001000".chars().collect::<Vec<char>>(), 2 - 1, 3 - 1), true);
	}
}
