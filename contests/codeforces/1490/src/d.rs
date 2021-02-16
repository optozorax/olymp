fn calc_depth(depth: &mut [usize], a: &[usize], pos: usize, inside: usize) {
	depth[pos] = inside;
	let left_max = a[0..pos]
		.iter()
		.enumerate()
		.max_by_key(|(_, x)| *x)
		.map(|(pos2, _)| calc_depth(&mut depth[0..pos], &a[0..pos], pos2, inside + 1))
		.unwrap_or(());
	let right_max = a[pos + 1..]
		.iter()
		.enumerate()
		.max_by_key(|(_, x)| *x)
		.map(|(pos2, _)| calc_depth(&mut depth[pos + 1..], &a[pos + 1..], pos2, inside + 1))
		.unwrap_or(());
}

#[fastio::fastio]
pub fn main() {
	let t = read!(usize);
	for _ in 0..t {
		let n = read!(usize);
		let a = readln!(usize);
		let mut depth = vec![0; n];
		let max_pos = a.iter().enumerate().max_by_key(|(_, x)| *x).unwrap().0;
		calc_depth(&mut depth, &a, max_pos, 0);
		println!("{}", SpaceVec(depth));
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
include!("../../../../templates/src/to_include/display/space_vec.rs");
