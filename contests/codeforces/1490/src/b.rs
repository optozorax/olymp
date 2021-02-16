#[fastio::fastio]
pub fn main() {
	let t = read!(usize);
	for _ in 0..t {
		let n = read!(i32);
		let a = readln!(i32).into_iter().map(|x| x % 3).collect::<Vec<i32>>();
		let x = n / 3;
		let c0 = a.iter().filter(|x| **x == 0).count() as i32;
		let c1 = a.iter().filter(|x| **x == 1).count() as i32;
		let c2 = a.iter().filter(|x| **x == 2).count() as i32;
		let f = |a3: i32| {
			let a2 = c1 + c0 + a3 - 2 * x;
			let a1 = c0 + a3 - x;
			let count = a1 + a2 + a3;
			if a2 < 0 || a1 < 0 { None } else { Some(count) }
		};
		let answer = binary_search(0..n as usize, |a3| f(a3 as i32).map(|x| x >= 0).unwrap_or(false)).unwrap_or(0);
		let count = f(answer as i32).unwrap_or(0);
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
