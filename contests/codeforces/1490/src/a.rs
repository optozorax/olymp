#[fastio::fastio]
pub fn main() {
	let t = read!(usize);
	for _ in 0..t {
		let n = read!(usize);
		let a = readln!(u32);
		let answer: u32 = a
			.windows(2)
			.map(|x| {
				let a = x[0];
				let b = x[1];
				let min = std::cmp::min(a, b);
				let max = std::cmp::max(a, b);
				let count = max / min;
				if min * 2 < max {
					let power_of_two = 31 - count.leading_zeros();
					let power = 1 << power_of_two;
					let is_has_additional = power * min == max;
					power_of_two - is_has_additional as u32
				} else {
					0
				}
			})
			.sum();
		println!("{}", answer);
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
