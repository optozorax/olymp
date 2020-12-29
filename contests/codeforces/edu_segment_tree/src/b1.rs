#[fastio::fastio]
pub fn main() {
	let _n = read!(i64);
	let m = read!(i64);
	let mut a = readln!(i64);
	let mut tree = SegmentTree::create(&mut a, |_, x, y| std::cmp::min(x, y), |_, x| x);
	for _ in 0..m {
		let c = read!(i64);
		let x = read!(i64);
		let y = read!(i64);
		if c == 1 {
			tree.set(x as usize, y);
		} else {
			println!("{}", tree.f_for_segment(x as usize..y as usize).unwrap());
		}
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
include!("../../../../templates/src/to_include/option.rs");
include!("../../../../templates/src/to_include/segment_tree.rs");
