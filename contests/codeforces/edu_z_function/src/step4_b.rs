#[fastio::fastio]
pub fn main() {
	let t = read!(usize);
	for _ in 0..t {
		let s = bytes!();
		let t = bytes!();
		if let Some(mut pos) = &t
			.iter()
			.chain(std::iter::once(&b'$'))
			.chain(s.iter())
			.chain(s.iter())
			.copied()
			.collect::<ZVec<u8>>()
			.z
			.into_iter()
			.position(|x| x == t.len())
		{
			pos -= t.len();
			pos -= 1;
			println!("{}", pos);
		} else {
			println!("-1");
		}
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
include!("../../../../templates/src/to_include/z_function.rs");
