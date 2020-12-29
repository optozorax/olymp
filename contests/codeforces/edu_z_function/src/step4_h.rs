fn solve(input: &[u8]) -> u64 {
	let mut is_unique: Vec<Vec<bool>> = (0..input.len()).map(|len| vec![true; input.len() - len]).collect();
	let prefix_s = input
		.iter()
		.copied()
		.chain(std::iter::once(b'$'))
		.chain(input.iter().copied())
		.collect::<Vec<_>>();
	let mut substring = vec![false; input.len()];
	for i in 0..input.len() {
		if substring[i] {
			continue;
		}
		let mut is_discovered = vec![false; input.len() - i];
		let zs = z_function(&prefix_s[i..]);
		let mut lower = 0;
		zs.iter()
			.skip(input.len() - i + 1)
			.enumerate()
			.filter(|(index, count)| *index + *count == input.len())
			.for_each(|(index, _)| substring[index] = true);
		let iter = zs.into_iter().skip(input.len() - i + 1).enumerate();
		for (index, count) in iter {
			for j in (lower..count).rev() {
				if !is_unique[j][index] {
					lower = j;
					break;
				}
				if is_discovered[j] {
					is_unique[j][index] = false;
				} else {
					is_discovered[j] = true;
				}
			}
		}
	}
	is_unique
		.into_iter()
		.enumerate()
		.map(|(index, x)| (index as u64 + 1u64) * x.into_iter().map(|x| x as u64).sum::<u64>())
		.sum()
}

#[fastio::fastio]
pub fn main() {
	let s = bytes!();
	println!("{}", solve(&s));
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
include!("../../../../templates/src/to_include/z_function.rs");
