use rand::prelude::*;

#[fastio::fastio]
pub fn main() {
	// -------------------------------------------------------------------- //
	//                      DO NOT FORGET TO FLUSH!!!                       //
	// -------------------------------------------------------------------- //

	let t = read!(usize);
	let mut cases = Vec::new();
	for _ in 0..t {
		let n = read!(usize);
		let k = read!(usize);
		cases.push((n, k));
	}

	println!("{}", t);
	for (n, k) in &cases {
		println!("{} {}", n, k);
	}
	flush!();

	for (n, k) in cases {
		let readed = bytes!();
		assert_eq!(readed.len(), n);
		for_each_subslice(&readed, |s| {
			if is_palindrome(s) {
				assert!(s.len() <= k);
			}
		});
	}

	eprintln!("OK");
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
include!("../../../../templates/src/to_include/for_each_subslice.rs");
include!("../../../../templates/src/to_include/palindrome.rs");
