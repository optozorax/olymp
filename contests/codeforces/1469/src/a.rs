fn solve(s: &mut [u8]) -> bool {
	let open = s.iter().position(|x| *x == b'(').unwrap();
	let close = s.iter().position(|x| *x == b')').unwrap();
	if open < close { s.len() % 2 == 0 } else { close > 0 && open < s.len() - 1 && s.len() % 2 == 0 }
}

#[fastio::fastio]
pub fn main() {
	let t = read!(usize);
	for _ in 0..t {
		let mut s = bytes!();
		if solve(&mut s) {
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
