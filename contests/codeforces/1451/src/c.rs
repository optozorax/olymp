#![allow(unused_imports, clippy::many_single_char_names)]
use std::{
	collections::BTreeMap,
	convert::TryFrom,
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

#[derive(Debug, Clone)]
struct PushMap<K, V>(pub BTreeMap<K, Vec<V>>);

impl<K: Ord, V> FromIterator<(K, V)> for PushMap<K, V> {
	fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> PushMap<K, V> {
		let mut result = BTreeMap::new();
		for (k, v) in iter.into_iter() {
			result.entry(k).or_insert_with(Vec::new).push(v);
		}
		PushMap(result)
	}
}

trait CountVecTrait {
	fn collect_count(self, max: usize) -> Vec<usize>;
}

impl<T: Iterator<Item = K>, K: Into<usize>> CountVecTrait for T {
	fn collect_count(self, max: usize) -> Vec<usize> {
		let mut result = vec![0; max];
		for i in self {
			result[i.into()] += 1;
		}
		result
	}
}

fn solve(a: &[u8], b: &[u8], k: usize) -> bool {
	let mut have = a.iter().copied().collect_count(27);
	let need = b.iter().copied().collect_count(27);

	for i in 0..26 {
		if (have[i] < need[i]) || ((have[i] - need[i]) % k != 0) {
			return false;
		}
		have[i] -= need[i];
		have[i + 1] += have[i];
	}

	true
}

pub fn main() {
	let t = read!(u32);
	for _ in 0..t {
		let n = read!(usize);
		let k = read!(usize);
		let a = read!([u8lowletter; n]);
		let b = read!([u8lowletter; n]);
		if solve(&a, &b, k) {
			println!("Yes");
		} else {
			println!("No");
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn chars<T: IntoIterator<Item = &'static u8>>(input: T) -> Vec<u8> { input.into_iter().map(|x| x - b'a').collect() }

	#[test]
	fn name() {
		assert_eq!(solve(&chars(b"abc"), &chars(b"bcd"), 3), false);
		assert_eq!(solve(&chars(b"abba"), &chars(b"azza"), 2), true);
		assert_eq!(solve(&chars(b"zz"), &chars(b"aa"), 1), false);
		assert_eq!(solve(&chars(b"aaabba"), &chars(b"ddddcc"), 2), true);
	}
}
