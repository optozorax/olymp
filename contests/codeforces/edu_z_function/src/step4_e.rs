#[fastio::fastio]
pub fn main() {
	let s = bytes!();
	let t = bytes!();
	if s.len() != t.len() {
		println!("No");
		return;
	}

	let z1 = t
		.iter()
		.rev()
		.copied()
		.chain(std::iter::once(b'$'))
		.chain(s.iter().copied())
		.collect::<ZVec<_>>()
		.z;
	let k_upper = z1[t.len() + 1];
	let z2 = t
		.iter()
		.copied()
		.chain(std::iter::once(b'$'))
		.chain(s.iter().copied())
		.collect::<ZVec<_>>()
		.z;
	let result = z2
		.into_iter()
		.skip(t.len() + 1)
		.enumerate()
		.filter(|(index, potential_k)| index + potential_k == s.len() && s.len() - *potential_k <= k_upper)
		.max_by_key(|(_, k)| *k);
	if let Some((_, k)) = result {
		println!("Yes\n{}", s.len() - k);
	} else {
		println!("No");
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
include!("../../../../templates/src/to_include/z_function.rs");
