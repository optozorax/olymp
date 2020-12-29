use rand::prelude::*;

#[fastio::fastio]
pub fn main() {
	// -------------------------------------------------------------------- //
	//                      DO NOT FORGET TO FLUSH!!!                       //
	// -------------------------------------------------------------------- //

	let t = read!(usize);
	println!("{}", t);
	for _ in 0..t {
		let n = read!(usize);
		let m = read!(usize);
		let a = readln!(usize);
		println!("{} {}\n{}", n, m, SpaceVec(a));
		for _ in 0..m {
			let r = read!(usize);
			let p = read!(f64);
			println!("{} {}", r, p);
		}
	}
	let mut probs = Vec::new();
	for _ in 0..t {
		probs.push(read!(f64));
	}
	flush!();
	for should_be in probs {
		let prob = read!(f64);
		assert!((should_be - prob).abs() <= 1e-6);
	}
	eprintln!("OK");
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
include!("../../../../templates/src/to_include/display/space_vec.rs");
