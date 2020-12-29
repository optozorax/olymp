fn solve(s: &[u8], t: &[u8]) -> usize {
	let mut is = vec![0u64; s.len()];
	for (index, is) in is.iter_mut().enumerate() {
		if s.get(index..index + t.len())
			.map(|x| x.iter().eq(t.iter()))
			.unwrap_or(false)
		{
			*is = 1;
		}
	}
	let sum = is
		.iter()
		.scan(0, |state, value| {
			*state += *value;
			Some(*state)
		})
		.collect::<Vec<_>>();

	let mut counter = 0;
	for i in 0..s.len() {
		counter += if is[i] == 1 {
			t.len() - 1
		} else if let Some(pos) = binary_search(i..s.len(), |pos| sum[pos] != sum[i]) {
			pos - i + t.len() - 1
		} else {
			s.len() - i
		};
	}
	counter
}

#[fastio::fastio]
pub fn main() {
	let t = read!(usize);
	for _ in 0..t {
		let s = bytes!();
		let t = bytes!();
		println!("{}", solve(&s, &t));
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/ \/ TESTING CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
//----------------------------------------------------------------------------

mod tests {
	use super::*;

	macro_rules! test {
		($name:ident, $s:literal, $t:literal, $answer:expr) => {
			#[test]
			fn $name() {
				assert_eq!(
					solve(&$s.iter().copied().collect::<Vec<u8>>(), &$t.iter().copied().collect::<Vec<u8>>()),
					$answer
				);
			}
		};
	}

	test!(a1, b"zaza", b"az", 6);
	test!(a2, b"abacaba", b"ab", 14);
	test!(a3, b"aaaaa", b"a", 0);
	test!(a4, b"aaa", b"b", 6);

	test!(b1, b"abbbbabbbabb", b"b", 3);
	test!(b2, b"ababbaa", b"ab", 14);
	test!(b3, b"az", b"az", 2);
	test!(b4, b"z", b"a", 1);
	test!(b5, b"a", b"aa", 1);
	test!(b6, b"ababaaba", b"aba", 19);
	test!(b7, b"abcdefgh", b"fgh", 30);
	test!(b8, b"abacabadaba", b"ba", 24);
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
include!("../../../../templates/src/to_include/binary_search/base.rs");
