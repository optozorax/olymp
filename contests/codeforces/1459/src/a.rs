#[fastio::fastio]
pub fn main() {
	let t = read!(usize);
	for _ in 0..t {
		let _n = read!(usize);
		let r = bytes!();
		let b = bytes!();
		let mut less_count = 0;
		let mut greater_count = 0;
		r.into_iter()
			.zip(b.into_iter())
			.map(|(x, y)| x.cmp(&y))
			.for_each(|o| match o {
				Ordering::Less => less_count += 1,
				Ordering::Equal => {},
				Ordering::Greater => greater_count += 1,
			});
		match less_count.cmp(&greater_count) {
			Ordering::Less => println!("RED"),
			Ordering::Equal => println!("EQUAL"),
			Ordering::Greater => println!("BLUE"),
		}
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
