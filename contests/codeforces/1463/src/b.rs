fn solve(a: &[u64]) -> Vec<u64> {
	a.iter()
		.map(|x| {
			let a = x.next_power_of_two();
			let b = a / 2;
			if (a - x) < (x - b) && a < 1_000_000_000 { a } else { b }
		})
		.collect()
}

#[fastio::fastio]
pub fn main() {
	let t = read!(usize);
	for _ in 0..t {
		let _n = read!(usize);
		let a = readln!(u64);
		println!("{}", SpaceVec(solve(&a)));
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
include!("../../../../templates/src/to_include/display/space_vec.rs");
