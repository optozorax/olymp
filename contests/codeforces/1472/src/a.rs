#[fastio::fastio]
pub fn main() {
	let t = read!(usize);
	for _ in 0..t {
		let mut w = read!(u64);
		let mut h = read!(u64);
		let n = read!(u64);

		let mut count_twos_h = 1;
		while h % 2 == 0 && h != 0 {
			h /= 2;
			count_twos_h *= 2;
		}

		let mut count_twos_w = 1;
		while w % 2 == 0 && w != 0 {
			w /= 2;
			count_twos_w *= 2;
		}

		if count_twos_w * count_twos_h >= n {
			println!("YES");
		} else {
			println!("NO");
		}
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
