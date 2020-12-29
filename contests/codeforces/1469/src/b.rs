#[fastio::fastio]
pub fn main() {
	let t = read!(usize);
	for _ in 0..t {
		let _n = read!(usize);
		let mut r = readln!(i32);
		let _m = read!(usize);
		let mut b = readln!(i32);
		r.push(0);
		b.push(0);

		let max1 = r.iter().scan_after(0, |acc, x| acc + *x).map(fst).max().unwrap();
		let max2 = b.iter().scan_after(0, |acc, x| acc + *x).map(fst).max().unwrap();

		println!("{}", max1 + max2);
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
include!("../../../../templates/src/to_include/iterator/fst_snd.rs");
include!("../../../../templates/src/to_include/iterator/scan_after.rs");
