fn solve(k: u64) -> Vec<u64> {
	if k == 0 {
		Vec::new()
	} else {
		let (count, sum) = (1u64..)
			.map(|n| (n, 2u64.pow(n as u32 + 1) - 2))
			.take_while(|(_, v)| *v <= k)
			.last()
			.unwrap();
		let mut result = solve(k - sum);
		result.push(count);
		result
	}
}

#[fastio::fastio]
pub fn main() {
	let t = read!(i64);
	for _ in 0..t {
		let k = read!(u64);
		if k % 2 == 1 {
			println!("-1");
		} else {
			let answer = solve(k);
			let len = answer.iter().sum::<u64>();
			println!("{}", len);
			if len > 2000 {
				println!("-1");
			} else {
				for i in answer.iter().rev() {
					print!("1 ");
					for _ in 0..i - 1 {
						print!("0 ");
					}
				}
				println!();
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
