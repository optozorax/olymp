#[fastio::fastio]
pub fn main() {
	let t = read!(usize);
	for _ in 0..t {
		let _n = read!(usize);
		let b = readln!(i64);
		let mut c = &b[..];
		let mut left = true;
		let mut a = Vec::new();
		while !c.is_empty() {
			if left {
				a.push(c[0]);
				c = &c[1..];
			} else {
				a.push(*c.last().unwrap());
				c = &c[..c.len() - 1];
			}
			left = !left;
		}
		println!("{}", SpaceVec(a));
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
include!("../../../../templates/src/to_include/display/space_vec.rs");
