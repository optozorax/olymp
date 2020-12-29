fn solve(s: &[u8]) -> bool {
	if s.len() < 4 {
		return false;
	}
	let year = b"2020";
	let delete_count = s.len() - 4;
	for i in 0..=s.len() - delete_count {
		if s.iter()
			.take(i)
			.chain(s.iter().skip(i + delete_count))
			.copied()
			.eq(year.iter().copied())
		{
			return true;
		}
	}
	false
}

#[fastio::fastio]
pub fn main() {
	let t = read!(usize);
	for _ in 0..t {
		let _n = read!(usize);
		let s = bytes!();
		if solve(&s) {
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

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
