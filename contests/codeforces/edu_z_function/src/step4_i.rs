fn solve(p: &[u8], t: &[u8], k: usize) -> Vec<usize> {
	let zpt = p
		.iter()
		.copied()
		.chain(once(b'$'))
		.chain(t.iter().copied())
		.collect::<ZVec<_>>()
		.z
		.into_iter()
		.skip(p.len() + 1);
	let zipt = p
		.iter()
		.rev()
		.copied()
		.chain(once(b'$'))
		.chain(t.iter().rev().copied())
		.collect::<ZVec<_>>()
		.z
		.into_iter()
		.skip(p.len() + 1)
		.collect::<Vec<_>>();

	zpt.enumerate()
		.filter(|(index, _)| index + p.len() <= t.len())
		.filter(|(index, v)| zipt[t.len() - (index + p.len())] + v >= p.len().saturating_sub(k))
		.map(|(index, _)| index + 1)
		.collect()
}

#[fastio::fastio]
pub fn main() {
	let t = bytes!();
	let p = bytes!();
	let k = read!(usize);
	let answer = solve(&p, &t, k);
	println!("{}", answer.len());
	println!("{}", SpaceVec(answer));
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
include!("../../../../templates/src/to_include/z_function.rs");
include!("../../../../templates/src/to_include/display/space_vec.rs");
