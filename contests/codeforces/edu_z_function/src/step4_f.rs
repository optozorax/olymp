fn solve(s: &[u8], t: &[u8]) -> Vec<u8> {
	let ts = t
		.iter()
		.copied()
		.chain(std::iter::once(b'$'))
		.chain(s.iter().copied())
		.collect::<ZVec<_>>();
	let st = s
		.iter()
		.copied()
		.chain(std::iter::once(b'$'))
		.chain(t.iter().copied())
		.collect::<ZVec<_>>();
	if ts.z.iter().skip(t.len() + 1).any(|z| *z == t.len()) {
		s.iter().copied().collect()
	} else if st.z.iter().skip(s.len() + 1).any(|z| *z == s.len()) {
		t.iter().copied().collect()
	} else if let Some((is_second, k)) =
		ts.z.iter()
			.skip(t.len() + 1)
			.enumerate()
			.filter(|(index, k)| *index + **k == s.len())
			.map(|(_, k)| (false, k))
			.chain(
				st.z.iter()
					.skip(s.len() + 1)
					.enumerate()
					.filter(|(index, k)| *index + **k == t.len())
					.map(|(_, k)| (true, k)),
			)
			.max_by_key(|(_, k)| *k)
	{
		if is_second {
			t.iter().chain(s.iter().skip(*k)).copied().collect()
		} else {
			s.iter().chain(t.iter().skip(*k)).copied().collect()
		}
	} else {
		t.iter().chain(s.iter()).copied().collect()
	}
}

#[fastio::fastio]
pub fn main() {
	let t = read!(usize);
	for _ in 0..t {
		let s = bytes!();
		let t = bytes!();
		println!("{}", Chars(solve(&s, &t)));
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
include!("../../../../templates/src/to_include/display/chars.rs");
include!("../../../../templates/src/to_include/z_function.rs");
