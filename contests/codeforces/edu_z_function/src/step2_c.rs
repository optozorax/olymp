fn solve(z: &[usize]) -> Option<Vec<usize>> {
	if z.is_empty() {
		return None;
	}
	if z[0] != 0 {
		return None;
	}
	let mut result: Vec<Option<usize>> = vec![None; z.len()];
	let mut symbol_not_equals: Vec<Vec<usize>> = vec![vec![]];
	*result.get_mut(0)? = Some(0);
	for (pos, equals) in z.iter().enumerate().skip(1) {
		for equals_pos in 0..*equals {
			let start_symbol = *result.get(equals_pos)?.as_ref()?;
			if let Some(symbol) = *result.get(equals_pos + pos)? {
				if symbol == start_symbol {
					// do nothing, everything is great
				} else if symbol_not_equals.get(start_symbol)?.iter().any(|x| *x == symbol) {
					// it must not equal to this symbol
					return None;
				} else {
					// merge two symbols
					result
						.iter_mut()
						.filter_map(|x| x.as_mut())
						.filter(|x| **x == symbol)
						.for_each(|x| *x = start_symbol);
					let to_extend = symbol_not_equals.get(symbol)?.clone();
					symbol_not_equals.get_mut(start_symbol)?.extend(to_extend);
				}
			} else {
				*result.get_mut(equals_pos + pos)? = Some(start_symbol);
			}
		}
		for not_equals_pos in (*equals)..min(equals + 1, result.len() - pos) {
			let start_symbol = *result.get(not_equals_pos)?.as_ref()?;
			if let Some(symbol) = result.get(not_equals_pos + pos)? {
				if *symbol == start_symbol {
					// it must not equals to this symbol
					return None;
				} else if symbol_not_equals.get(start_symbol)?.iter().any(|x| x == symbol) {
					// do nothing, everything is great
				} else {
					// add it to be not equals to this symbol
					symbol_not_equals.get_mut(start_symbol)?.push(*symbol);
				}
			} else {
				let new_number = symbol_not_equals.len();
				symbol_not_equals.get_mut(start_symbol)?.push(new_number);
				*result.get_mut(not_equals_pos + pos)? = Some(new_number);
				symbol_not_equals.push(vec![]);
			}
		}
	}
	Some(result.into_iter().map(|x| x.unwrap()).collect::<Vec<_>>())
}

#[fastio::fastio]
pub fn main() {
	let t = read!(usize);
	for _ in 0..t {
		let _n = read!(u64);
		let z = readln!(usize);
		if let Some(result) = solve(&z) {
			println!(
				"{}",
				result
					.into_iter()
					.map(|x| if (x as u8) <= b'z' - b'a' { x as u8 + b'a' } else { x as u8 - (b'z' - b'a') + b'A' })
					.map(char::from)
					.collect::<String>()
			)
		} else {
			println!("!");
		}
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/ \/ TESTING CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
//----------------------------------------------------------------------------

fn z_function_not_optimal<T: Eq>(input: &[T]) -> Vec<usize> {
	std::iter::once(0)
		.chain((1..input.len()).map(|i| {
			input
				.iter()
				.zip(input[i..].iter())
				.enumerate()
				.take_while(|(_, (a, b))| a == b)
				.map(|(index, _)| index + 1)
				.last()
				.unwrap_or(0)
		}))
		.collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
	use super::*;

	fn test(input: &str) {
		let input_new = input.chars().collect::<Vec<_>>();
		let z = z_function_not_optimal(&input_new);
		//dbg!(z);
		let new_array = solve(&z).expect(input);
		let z_new_array = z_function_not_optimal(&new_array);
		if z_new_array != z {
			dbg!(input);
			assert_eq!(z_new_array, z);
		}
	}

	fn test_wrong(z: &[usize]) {
		assert!(solve(z).is_none());
	}

	#[test]
	fn name() {
		test_wrong(&[50]);
		test_wrong(&[1, 1]);
		test_wrong(&[0, 2, 2, 1]);

		test("testet");
		test("abacabazabacaba");
		test("abcdef");
		test("eeeeee");
		test("axsyrnchouynchxxhaaaabaaxsy");
		test("aaaabaaaacabbcacccbbbccbbaaccccccabcaaccabcb");

		use rand::Rng;
		let mut rng = rand::thread_rng();
		for _ in 0..200000000 {
			let arr = (1..30)
				.map(|_| char::from(rng.gen_range(b'a', b'd' + 1)))
				.collect::<String>();

			test(&arr);
		}
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
