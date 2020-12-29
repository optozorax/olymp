#[fastio::fastio]
pub fn main() {
	let t = read!(usize);
	for _ in 0..t {
		let n = read!(usize);
		let arr = readln!(usize);
		let mut count = vec![0; n + 1];
		for i in &arr {
			count[*i] += 1;
		}
		if let Some((index, _)) = arr
			.into_iter()
			.enumerate()
			.filter(|(_, value)| count[*value] == 1)
			.min_by_key(|(_, value)| *value)
		{
			println!("{}", index + 1);
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
