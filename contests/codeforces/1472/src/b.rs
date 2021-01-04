#[fastio::fastio]
pub fn main() {
	let t = read!(usize);
	for _ in 0..t {
		let _n = read!(u64);
		let a = readln!(usize);
		let ones = a.iter().filter(|x| **x == 1).count();
		let twos = a.iter().filter(|x| **x == 2).count();
		if twos % 2 == 0 {
			if ones % 2 == 0 {
				println!("YES");
			} else {
				println!("NO");
			}
		} else {
			if ones % 2 == 0 {
				if ones >= 2 {
					println!("YES");
				} else {
					println!("NO");
				}
			} else {
				println!("NO");
			}
		}
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
