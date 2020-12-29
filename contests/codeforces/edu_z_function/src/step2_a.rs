#[fastio::fastio]
pub fn main() {
	let s = bytes!();
	let z = std::iter::once(0)
		.chain((1..s.len()).map(|i| {
			s.iter()
				.zip(s[i..].iter())
				.enumerate()
				.take_while(|(_, (a, b))| a == b)
				.map(|(index, _)| index + 1)
				.last()
				.unwrap_or(0)
		}))
		.collect::<Vec<_>>();
	println!("{}", SpaceVec(z));
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
include!("../../../../templates/src/to_include/display/space_vec.rs");
