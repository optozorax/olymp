#[fastio::fastio]
pub fn main() {
	let t = read!(usize);
	for _ in 0..t {
		let s = bytes!();
		let answer = (0..=s.len())
			.map(|pos| &s[0..pos])
			.filter(|s| s.iter().eq(s.iter().rev()))
			.last()
			.unwrap();
		println!("{}", answer.len());
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
