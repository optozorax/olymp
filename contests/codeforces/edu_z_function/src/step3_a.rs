#[fastio::fastio]
pub fn main() {
	let s = bytes!();
	println!("{}", SpaceVec(z_function(&s)));
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
include!("../../../../templates/src/to_include/display/space_vec.rs");
include!("../../../../templates/src/to_include/z_function.rs");
