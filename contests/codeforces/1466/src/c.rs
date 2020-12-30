#[fastio::fastio]
pub fn main() {
	let t = read!(usize);
	for _ in 0..t {
		let s = bytes!();
		if s.len() == 1 {
			println!("0");
		} else if s.len() == 2 {
			println!("{}", (s[0] == s[1]) as usize);
		} else {
			let mut changed = vec![false; s.len()];
			for i in 1..s.len() {
				changed[i] = (s[i] == s[i - 1] && !changed[i - 1]) || (i > 1 && s[i] == s[i - 2] && !changed[i - 2]);
			}
			println!("{}", changed.iter().filter(|x| **x).count());
		}
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
