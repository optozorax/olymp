fn solve(a: &[u8], b: &[u8], k: usize) -> bool {
	let mut have = a.iter().copied().map(|x| x - b'a').collect_count(27);
	let need = b.iter().copied().map(|x| x - b'a').collect_count(27);

	for i in 0..26 {
		if (have[i] < need[i]) || ((have[i] - need[i]) % k != 0) {
			return false;
		}
		have[i] -= need[i];
		have[i + 1] += have[i];
	}

	true
}

#[fastio::fastio]
pub fn main() {
	let t = read!(u32);
	for _ in 0..t {
		let _n = read!(usize);
		let k = read!(usize);
		let a = bytes!();
		let b = bytes!();
		if solve(&a, &b, k) {
			println!("Yes");
		} else {
			println!("No");
		}
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
include!("../../../../templates/src/to_include/iterator/count.rs");
