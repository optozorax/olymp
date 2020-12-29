fn solve(mut x: i64, mut a: Vec<i64>) -> Option<usize> {
	let mut count = 0;
	loop {
		let mut copy = a.clone();
		copy.sort_unstable();
		if copy == a {
			return Some(count);
		}
		count += 1;
		let pos = a
			.iter()
			.copied()
			.enumerate()
			.filter(|(_, i)| *i > x)
			.find(|(index, _)| {
				index
					.checked_sub(1)
					.and_then(|i| a.get(i))
					.map(|i| *i <= x)
					.unwrap_or(true) || a.get(index + 1).map(|i| x <= *i).unwrap_or(true)
			})?
			.0;
		std::mem::swap(&mut a[pos], &mut x);
	}
}

#[fastio::fastio]
pub fn main() {
	let t = read!(usize);
	for _ in 0..t {
		let _n = read!(i64);
		let x = read!(i64);
		let a = readln!(i64);
		if let Some(count) = solve(x, a) {
			println!("{}", count);
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
