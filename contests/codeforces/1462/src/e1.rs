#[fastio::fastio]
pub fn main() {
	let k = 2;
	let m = 3;

	let t = read!(usize);
	for _ in 0..t {
		let _ = read!(usize);
		let mut a = readln!(i64);
		a.sort_unstable();
		let mut count = 0;
		for (index, value) in a.iter().enumerate() {
			if let Some(pos) = binary_search(index + m - 1..a.len(), |pos| a[pos] - value > k)
				.unwrap_or(a.len())
				.checked_sub(1)
				.filter(|x| *x >= index + m - 1)
			{
				count += ArithmeticProgresion::new_canonical().sum_to((pos - index - 1) as i64);
			}
		}
		println!("{}", count);
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
include!("../../../../templates/src/to_include/binary_search/base.rs");
include!("../../../../templates/src/to_include/quadratic_equation.rs");
include!("../../../../templates/src/to_include/arithmetic_progression.rs");
