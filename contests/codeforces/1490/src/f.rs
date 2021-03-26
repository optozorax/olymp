fn count_numbers(a: &[u64]) -> Vec<u64> {
	let mut map: BTreeMap<u64, u64> = BTreeMap::new();
	for i in a {
		*map.entry(*i).or_insert(0) += 1;
	}
	let mut a = map.into_iter().map(|(_, x)| x).collect::<Vec<_>>();
	a.sort_unstable();
	a
}

fn solve(a: &[u64], is_fast: bool) -> u64 {
	let a = count_numbers(a);

	let a_not_equal = a
		.iter()
		.copied()
		.collect::<BTreeSet<_>>()
		.into_iter()
		.collect::<Vec<_>>();

	let f = |i| {
		let need = a_not_equal[i];
		let count: u64 = a.iter().map(|x| if *x < need { *x } else { *x - need }).sum();
		count
	};

	dbg!(&a_not_equal);
	for i in 0..a_not_equal.len() {
		eprint!("{} ", f(i));
	}
	eprintln!();

	for i in 0..a_not_equal.len() - 1 {
		eprint!("{} ", f(i) < f(i + 1));
	}
	eprintln!();

	if is_fast {
		parabola_eq_min(0..a_not_equal.len(), |i| f(i)).unwrap().1
	} else {
		(0..a_not_equal.len()).map(|i| f(i)).min().unwrap()
	}
}

#[fastio::fastio]
pub fn main() {
	let t = read!(usize);
	for _ in 0..t {
		let _n = read!(usize);
		let a = readln!(u64);
		println!("{}", solve(&a, true));
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/ \/ TESTING CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
//----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn name() {
		use rand::Rng;
		let mut rng = rand::thread_rng();
		for _ in 0..200000 {
			let size = 20;
			let a = (0..size).map(|_| rng.gen_range(1, 10)).collect::<Vec<u64>>();
			let fast_ans = solve(&a, true);
			let long_ans = solve(&a, false);
			if fast_ans != long_ans {
				dbg!(a, fast_ans, long_ans);
				panic!();
			}
		}
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
include!("../../../../templates/src/to_include/binary_search/parabola_eq_min.rs");
