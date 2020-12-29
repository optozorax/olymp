use rand::prelude::*;

#[fastio::fastio]
pub fn main() {
	// -------------------------------------------------------------------- //
	//                      DO NOT FORGET TO FLUSH!!!                       //
	// -------------------------------------------------------------------- //

	let t = read!(usize);
	println!("{}", t);
	let mut vec = Vec::new();
	for _ in 0..t {
		let n = read!(usize);
		let a = readln!(u64);
		println!("{}\n{}", n, SpaceVec(a.clone()));
		vec.push(a);
	}
	flush!();

	for v in vec {
		let new_vec = readln!(u64);
		assert!(v.len() == new_vec.len());
		assert!(new_vec.iter().all(|x| 1 <= *x && *x <= 1_000_000_000));
		assert!(
			new_vec
				.windows(2)
				.map(|x| (x[0], x[1]))
				.all(|(a, b)| a % b == 0 || b % a == 0)
		);
		let s = v.iter().sum::<u64>();
		assert!(
			new_vec
				.iter()
				.copied()
				.zip(v.iter().copied())
				.map(|(x, y)| (x as i64 - y as i64).abs() as u64)
				.sum::<u64>() * 2
				<= s
		);
	}
	eprintln!("OK");
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
include!("../../../../templates/src/to_include/display/space_vec.rs");
