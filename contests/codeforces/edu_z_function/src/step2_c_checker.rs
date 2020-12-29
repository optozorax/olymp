use rand::prelude::*;

#[fastio::fastio]
pub fn main() {
	// -------------------------------------------------------------------- //
	//                      DO NOT FORGET TO FLUSH!!!                       //
	// -------------------------------------------------------------------- //

	let t = read!(usize);
	let mut zs = Vec::new();
	println!("{}", t);
	for _ in 0..t {
		let n = read!(u64);
		println!("{}", n);
		let z = readln!(usize);
		println!("{}", SpaceVec(z.clone()));
		zs.push(z);
	}
	flush!();

	for i in &mut zs {
		let s = bytes!();
		if s == b"NO" {
			i.clear();
		}
	}

	for z in zs {
		let s = bytes!();
		if z.is_empty() {
			assert_eq!(s, b"!");
		} else {
			let z_of_s = z_function(&s);
			assert_eq!(z, z_of_s);
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
include!("../../../../templates/src/to_include/display/space_vec.rs");
include!("../../../../templates/src/to_include/z_function.rs");
