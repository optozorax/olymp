#[fastio::fastio]
pub fn main() {
	let t = read!(usize);
	for _ in 0..t {
		let _n = read!(usize);
		let mut p = read!(usize);
		let k = read!(usize);
		p -= 1;
		let a = bytes!().into_iter().map(|x| x == b'1').collect::<Vec<_>>();
		let x = read!(usize);
		let y = read!(usize);

		let mut count = vec![0; a.len() - p];
		for i in (p..a.len()).rev() {
			count[i - p] = count.get(i - p + k).copied().unwrap_or(0) + !a[i] as usize;
		}

		let answer = count
			.iter()
			.enumerate()
			.map(|(index, value)| index * y + value * x)
			.min()
			.unwrap();
		println!("{}", answer);
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
