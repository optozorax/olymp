#[fastio::fastio]
pub fn main() {
	let _n = read!(i64);
	let _m = read!(i64);
	let mut a = readln!(i64);
	let mut b = readln!(i64);
	a.sort_unstable();
	let max_gcd = a.windows(2).map(|w| w[1] - w[0]).fold(0, gcd);
	b.iter_mut().for_each(|x| *x = gcd(*x + a[0], max_gcd));
	println!("{}", SpaceVec(b));
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/ \/ TESTING CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
//----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn name() {
		use rand::{seq::SliceRandom, Rng};
		let mut rng = rand::thread_rng();
		for _ in 0..200000 {
			let size = rng.gen_range(5, 10);
			let mut vec = (0..size).map(|_| rng.gen_range(1, 1000)).collect::<Vec<i64>>();
			vec.sort_unstable();
			let max_gcd = vec.windows(2).map(|w| w[1] - w[0]).fold(0, gcd);
			for i in 0..1000 {
				let gcd1 = vec.iter().fold(0, |acc, x| gcd(acc, x + i));
				let gcd2 = gcd(max_gcd, i + vec[0]);
				// println!("i: {}, {} {}", i, gcd1, gcd2);
				if gcd1 != gcd2 {
					dbg!(vec, i, gcd1, gcd2);
					panic!();
				}
			}
			// dbg!((0..100000).map(|i| vec.iter().fold(0, |acc, x| gcd(acc, x+i))).collect::<BTreeSet<_>>());
			// dbg!(max_gcd);
		}
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
include!("../../../../templates/src/to_include/math/gcd.rs");
include!("../../../../templates/src/to_include/display/space_vec.rs");
