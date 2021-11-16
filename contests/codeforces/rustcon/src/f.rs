#[fastio::fastio]
pub fn main() {
	let ans = vec![(b"rgwb", 1), (b"bgwr", 2), (b"grbw", 3), (b"rwbg", 4), (b"wrgb", 5), (b"gbrw", 6)];
	let s = bytes!();
	for (t, ans) in &ans {
		for j in 0..4 {
			if s.iter().enumerate().all(|(k, s)| *s == t[(k + j) % 4]) {
				println!("{}", ans);
				return;
			}
		}
	}
	println!("-1");
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
