#[fastio::fastio]
pub fn main() {
	let t = read!(usize);
	for _ in 0..t {
		let t = bytes!();
		let p = bytes!();
		if p.len() > t.len() {
			println!("0");
			println!();
			continue;
		}
		let mut result = Vec::new();
		for i in 0..t.len() - p.len() + 1 {
			if t[i..i + p.len()]
				.iter()
				.zip(p.iter())
				.all(|(t, p)| if *p == b'?' { true } else { t == p })
			{
				result.push(i);
			}
		}
		println!("{}", result.len());
		println!("{}", SpaceVec(result));
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
include!("../../../../templates/src/to_include/display/space_vec.rs");
