#[fastio::fastio]
pub fn main() {
	let t = read!(i64);
	for _ in 0..t {
		let _n = read!(i64);
		let _m = read!(i64);
		let lower = readln!(i64);
		let left = readln!(i64);
		let answer = left.into_iter().filter_map(|x| lower.iter().find(|y| **y == x)).count();
		println!("{}", answer);
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
