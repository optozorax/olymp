#[fastio::fastio]
pub fn main() {
	let number = {
		let mut result = BTreeMap::new();
		for_each_subset(&[1, 2, 3, 4, 5, 6, 7, 8, 9], |iter| {
			let sum = iter.clone().sum::<usize>();
			let number = iter.clone().fold(0, |acc, x| acc * 10 + x);
			result
				.entry(sum)
				.and_modify(|x| *x = std::cmp::min(*x, number))
				.or_insert(number);
		});
		result
	};

	let t = read!(usize);
	for _ in 0..t {
		let x = read!(usize);
		if let Some(string) = number.get(&x) {
			println!("{}", string);
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
include!("../../../../templates/src/to_include/for_each_number/base.rs");
include!("../../../../templates/src/to_include/for_each_number/subset.rs");
