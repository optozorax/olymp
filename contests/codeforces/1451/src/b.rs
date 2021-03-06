fn solve(s: &[u8], l: usize, r: usize) -> bool {
	let first_pos = s.iter().position(|x| *x == s[l]).unwrap();
	let last_pos = s.iter().rev_position(|x| *x == s[r]).unwrap();
	first_pos != l || last_pos != r
}

#[fastio::fastio]
pub fn main() {
	let t = read!(u8);
	for _ in 0..t {
		let _n = read!(u8);
		let q = read!(u8);
		let s = bytes!();
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

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
include!("../../../../templates/src/to_include/iterator/rev_position.rs");
