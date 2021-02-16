use std::collections::HashSet;

#[fastio::fastio]
pub fn main() {
	let count = 10_020u64;
	let cubes = (1..count).map(|x| x * x * x).collect::<HashSet<_>>();
	let t = read!(usize);
	for _ in 0..t {
		let n = read!(u64);
		let has = (1..count).any(|i| {
			let i3 = i * i * i;
			let b3 = n - i3;
			cubes.contains(&b3)
		});
		if has {
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
