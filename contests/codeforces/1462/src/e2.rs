struct Solver {
	inverses: ModInverses,
	factorial: ModFactorials,
}

impl Solver {
	fn new() -> Self {
		let max_count = 3 * 100_000;
		let inverses = ModInverses::new(max_count);
		let factorial = ModFactorials::new(max_count, &inverses);
		Self { inverses, factorial }
	}

	fn solve(&self, a: &mut [i64], n: i64, m: usize, k: i64) -> Field {
		if m == 1 {
			Field::from(a.len())
		} else {
			a.sort_unstable();

			let prefix_sums = {
				let mut state = Field::from(0);
				(m - 2..=n as usize + m + 3)
					.map(Field::from)
					.map(|x| {
						state = state + self.factorial.get_choose(x, Field::from(m - 2));
						state
					})
					.collect::<Vec<_>>()
			};

			let mut count = Field::from(0);
			for (index, value) in a.iter().enumerate() {
				if let Some(pos) = binary_search(index + m - 1..a.len(), |pos| a[pos] - value > k)
					.unwrap_or(a.len())
					.checked_sub(1)
					.filter(|x| *x >= index + m - 1)
				{
					let size = pos - index - 1;
					count = count + prefix_sums[size - (m - 2)];
				}
			}
			count
		}
	}
}

#[fastio::fastio]
pub fn main() {
	let solver = Solver::new();

	let t = read!(usize);
	for _ in 0..t {
		let n = read!(i64);
		let m = read!(usize);
		let k = read!(i64);
		let mut a = readln!(i64);

		println!("{}", solver.solve(&mut a, n, m, k).val);
		// println!("{}", solve_slow(&a.0, n, m, k));
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/ \/ TESTING CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
//----------------------------------------------------------------------------

fn solve_slow(a: &[i64], _n: i64, m: usize, k: i64) -> usize {
	let mut count = 0;
	for_each_subset(a, |iter| {
		if iter.clone().count() == m && iter.clone().max().unwrap() - iter.clone().min().unwrap() <= k {
			count += 1;
		}
	});
	count
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn name() {
		let solver = Solver::new();
		use rand::{seq::SliceRandom, Rng};
		let mut rng = rand::thread_rng();
		for _ in 0..200000 {
			let n = 5;
			let mut a = (0..n).map(|_| rng.gen_range(1, n + 1)).collect::<Vec<i64>>();
			let m = rng.gen_range(1, n + 1);
			let k = rng.gen_range(1, n + 1);
			let fast_ans = solver.solve(&mut a, n, m as usize, k).val;
			let long_ans = solve_slow(&a, n, m as usize, k);
			if fast_ans != long_ans as i64 {
				dbg!(a, n, m, k, fast_ans, long_ans);
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
include!("../../../../templates/src/to_include/for_each_number/base.rs");
include!("../../../../templates/src/to_include/for_each_number/subset.rs");
include!("../../../../templates/src/to_include/mod_arithmetic.rs");
include!("../../../../templates/src/to_include/binary_search/base.rs");
