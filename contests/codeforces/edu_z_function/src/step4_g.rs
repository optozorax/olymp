fn solve(input: &[usize]) -> Vec<usize> {
	let halflen = input.len() / 2;
	let ss = input
		.iter()
		.copied()
		.take(halflen)
		.chain(std::iter::once(0))
		.chain(input.iter().rev().copied())
		.collect::<ZVec<_>>();
	let mut result =
		ss.z.iter()
			.zip(ss.vec.iter())
			.skip(halflen + 1)
			.enumerate()
			.filter(|(index, (z, _))| (input.len() - index) % 2 == 0 && **z >= (input.len() - index) / 2)
			.map(|(index, _)| input.len() - (input.len() - index) / 2)
			.collect::<Vec<_>>();
	result.push(input.len());
	result
}

#[fastio::fastio]
pub fn main() {
	let _ = read!(usize);
	let _ = read!(usize);
	let s = readln!(usize);
	println!("{}", SpaceVec(solve(&s)));
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/ \/ TESTING CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
//----------------------------------------------------------------------------

fn long_solve(input: &[usize]) -> Vec<usize> {
	let mut result = Vec::new();
	for i in (1..=input.len() / 2 + 1).rev() {
		if input[0..i].iter().rev().eq(input[i..].iter().take(i)) {
			result.push(input.len() - i);
		}
	}
	result.push(input.len());
	result
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn name() {
		use rand::{seq::SliceRandom, Rng};
		let mut rng = rand::thread_rng();
		for _ in 0..2000000 {
			//let mut arr = (0..30).map(|x| x + 1).collect::<Vec<usize>>();
			let mut arr = (0..10).map(|_| rng.gen_range(1, 10)).collect::<Vec<usize>>();
			arr.shuffle(&mut rng);
			let fast_ans = solve(&arr);
			let long_ans = long_solve(&arr);
			if fast_ans != long_ans {
				dbg!(arr, fast_ans, long_ans);
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
include!("../../../../templates/src/to_include/display/space_vec.rs");
include!("../../../../templates/src/to_include/z_function.rs");
