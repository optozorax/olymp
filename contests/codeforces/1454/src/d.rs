#[allow(clippy::mut_range_bound)]
fn factorize(mut n: u64) -> BTreeMap<u64, usize> {
	let mut result = BTreeMap::new();
	for i in 2..n {
		if i > n || i > (n as f64).sqrt() as u64 + 5 {
			break;
		}
		while n % i == 0 {
			*result.entry(i).or_insert(0) += 1;
			n /= i;
		}
	}
	if n != 0 {
		*result.entry(n).or_insert(0) += 1;
	}
	result
}

#[fastio::fastio]
pub fn main() {
	let t = read!(usize);
	for _ in 0..t {
		let mut n = read!(u64);
		let max_count = factorize(n)
			.into_iter()
			.max_by_key(|(_miltiprier, count)| *count)
			.unwrap();
		println!("{}", max_count.1);
		for _ in 0..max_count.1 - 1 {
			print!("{} ", max_count.0);
			n /= max_count.0;
		}
		println!("{}", n);
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
