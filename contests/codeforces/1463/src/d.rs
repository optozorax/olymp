fn calc(a: &[usize], b: &[usize]) -> usize {
	let mut count = 0;
	let mut set = b.iter().copied().collect::<BTreeSet<usize>>();
	for i in a {
		let min = *set.iter().next().unwrap();
		if min > *i {
			count += 1;
			set.remove(&set.iter().rev().next().unwrap().clone());
		} else {
			set.remove(&min);
		}
	}
	count
}

#[fastio::fastio]
pub fn main() {
	let t = read!(usize);
	for _ in 0..t {
		let n = read!(usize);
		let a = readln!(usize);
		let b = (1..=2 * n)
			.filter(|x| a.binary_search(x).is_err())
			.collect::<Vec<usize>>();

		let min = calc(&a, &b);
		let max = calc(&b, &a);
		let answer = ((n - max) as i64 - min as i64).abs() + 1;
		println!("{}", answer);
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
