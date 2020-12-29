#[fastio::fastio]
pub fn main() {
	let t = read!(usize);
	for _ in 0..t {
		let n = read!(usize);
		if n % 2 == 0 {
			(1..=n).rev().for_each(|x| print!("{} ", x));
		} else {
			(1..=n).rev().for_each(|x| {
				let result = if n / 2 + 1 == x {
					x + 1
				} else if n / 2 + 2 == x {
					x - 1
				} else {
					x
				};
				print!("{} ", result);
			})
		}
		println!();
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
