use rand::prelude::*;

#[fastio::fastio]
pub fn main() {
	// -------------------------------------------------------------------- //
	//                      DO NOT FORGET TO FLUSH!!!                       //
	// -------------------------------------------------------------------- //

	let mut input = Vec::new();

	let t = read!(usize);
	println!("{}", t);
	for _ in 0..t {
		let n = read!(i64);
		let c = bytes!();
		input.push(c.clone());
		println!("{}\n{}", n, Chars(c));
	}
	flush!();

	for mut vec in input {
		let mut ans = bytes!();
		let has_no_trygub = ans.windows(b"trygub".len()).all(|x| x != b"trygub");
		let same_permutation = {
			ans.sort_unstable();
			vec.sort_unstable();
			ans == vec
		};
		if !has_no_trygub || !same_permutation {
			eprintln!("ERR");
			return;
		}
	}
	eprintln!("OK");
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
include!("../../../../templates/src/to_include/display/chars.rs");
