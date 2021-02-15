#[fastio::fastio]
pub fn main() {
	let all = (1..)
		.map(|x| x * 2 + 1)
		.map(|a| (a * a - 1) / 2)
		.map(|b| b + 1)
		.take_while(|c| *c < 1_000_000_100)
		.collect::<Vec<u64>>();
	let t = read!(usize);
	for _ in 0..t {
		let n = read!(u64);
		let pos = all.binary_search(&n).map(|x| x + 1).unwrap_or_else(|pos| pos);
		println!("{}", pos);
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
