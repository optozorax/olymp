#[fastio::fastio]
pub fn main() {
	let t = read!(usize);
	for _ in 0..t {
		let n = read!(usize);
		let a = readln!(usize);
		let mut set = BTreeSet::new();
		for i in 0..a.len() {
			for j in i + 1..a.len() {
				set.insert(a[j] - a[i]);
			}
		}
		println!("{}", set.len());
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
