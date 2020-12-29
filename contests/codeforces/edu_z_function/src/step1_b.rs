#[fastio::fastio]
pub fn main() {
	let t = read!(usize);
	for _ in 0..t {
		let s = bytes!();
		let mut counter = 0;
		for i in 0..s.len() {
			for j in i + 1..=s.len() {
				let len = j - i;
				let prefix = s[0..len] == s[i..j];
				let suffix = s[s.len() - len..s.len()] == s[i..j];
				if (prefix || suffix) && !(prefix && suffix) {
					counter += 1;
				}
			}
		}
		println!("{}", counter);
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
