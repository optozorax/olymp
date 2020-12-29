#[fastio::fastio]
pub fn main() {
	let t = read!(usize);
	for _ in 0..t {
		let s = bytes!();
		if let Some((pos, _)) = z_function(&s)
			.into_iter()
			.enumerate()
			.filter(|(index, value)| index + value == s.len())
			.max_by_key(|(_, v)| *v)
		{
			println!("{}", Chars(s[0..pos].to_vec()));
		} else {
			println!("{}", Chars(s));
		}
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
