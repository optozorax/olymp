fn solve(s: &[u8]) -> usize {
	s.iter()
		.copied()
		.chain(once(b'$'))
		.chain(s.iter().copied())
		.chain(s.iter().copied())
		.collect::<ZVec<_>>()
		.z
		.into_iter()
		.skip(s.len() + 1)
		.take(s.len())
		.enumerate()
		.filter(|(_, v)| *v != s.len())
		.filter(|(index, v)| s[(index + v) % s.len()] < s[*v])
		.count() + 1
}

#[fastio::fastio]
pub fn main() {
	let s = bytes!();
	println!("{}", solve(&s));
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
include!("../../../../templates/src/to_include/z_function.rs");
