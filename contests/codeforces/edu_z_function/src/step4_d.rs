#[fastio::fastio]
pub fn main() {
	let s = bytes!();
	println!(
		"{}",
		s.iter()
			.copied()
			.chain(std::iter::once(b'$'))
			.chain(s.iter().copied().rev())
			.collect::<ZVec<_>>()
			.z
			.into_iter()
			.skip(s.len() + 1)
			.enumerate()
			.filter(|(index, z)| index + z == s.len())
			.max_by_key(|(_, z)| *z)
			.unwrap()
			.1
	);
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
include!("../../../../templates/src/to_include/z_function.rs");
