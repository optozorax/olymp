fn solve(k: u64, j: u64) -> u64 {
	let size = 2u64.pow(k as u32) - 1;
	if j == size / 2 {
		0
	} else if j == size / 2 + 1 {
		2u64.pow(k as u32 - 1) - 1
	} else if j > size / 2 {
		solve(k - 1, j - size / 2 - 1)
	} else {
		solve(k - 1, j)
	}
}

#[fastio::fastio]
pub fn main() {
	let t = read!(usize);
	for _ in 0..t {
		let k = read!(u64);
		let j = read!(u64);
		println!("{}", solve(k, j));
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
