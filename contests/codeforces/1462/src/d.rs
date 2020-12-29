#[fastio::fastio]
pub fn main() {
	let t = read!(usize);
	for _ in 0..t {
		let n = read!(u64);
		let a = readln!(i32);
		let sum = Prefix::new(&a, |x, y| x + y);
		let mut min = n - 1;
		'outer: for i in 0..a.len() {
			let current = sum.get_prefix_f(i + 1).unwrap();
			let mut counter = 1;
			let mut j = i + 1;
			while j < a.len() {
				if let Some(pos) = binary_search_number_leftmost(j..a.len() + 1, current, |pos| {
					sum.get_segment_f(j..pos, |x, y| x - y).unwrap()
				}) {
					j = pos;
				} else {
					continue 'outer;
				}
				counter += 1;
			}
			min = std::cmp::min(min, a.len() as u64 - counter);
		}
		println!("{}", min);
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
include!("../../../../templates/src/to_include/binary_search/base.rs");
include!("../../../../templates/src/to_include/binary_search/number.rs");
include!("../../../../templates/src/to_include/prefix.rs");
