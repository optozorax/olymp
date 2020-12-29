#[fastio::fastio]
pub fn main() {
	let t = read!(usize);
	for _ in 0..t {
		let s = bytes!();
		let mut count = z_function(&s)
			.into_iter()
			.collect_count(s.len())
			.into_iter()
			.rev()
			.scan_after(1, |state, x| state + x)
			.map(fst)
			.collect::<Vec<_>>();
		count.reverse();
		println!("{}", SpaceVec(count));
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
include!("../../../../templates/src/to_include/z_function.rs");
include!("../../../../templates/src/to_include/display/space_vec.rs");
include!("../../../../templates/src/to_include/iterator/scan_after.rs");
include!("../../../../templates/src/to_include/iterator/count.rs");
include!("../../../../templates/src/to_include/iterator/fst_snd.rs");
