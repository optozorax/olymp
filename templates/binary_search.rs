// It is supposed that `f` returns `false` `n` times, then `true` `n` times. This method uses binary search to find `n`. This function calls `f` only on range [on.start; on.end).
fn binary_search<F: Fn(usize) -> bool>(on: Range<usize>, f: F) -> Option<usize> {
	let mut a = on.start;
	let mut b = on.end;
	match b.checked_sub(a)? {
		0 => None,
		1 => Some(a).filter(|&x| f(x)),
		_ => if f(a) {
			Some(a)
		} else if !f(b-1) {
			None
		} else {
			b -= 1;
			loop {
				let c = a+(b-a)/2;
				match b-a {
					0 => unreachable!(),
					1 => return Some(b),
					2 => return Some(if f(c) { c } else { b }),
					_ => if f(c) { b = c; } else { a = c; }
				}
			}
		}
	}
}

// Finds leftmost position in `on` where number are equals to `number`. `f` must be monotone function.
fn binary_search_number_leftmost<F: Fn(usize) -> T, T: Ord + Eq + Copy + Debug>(on: Range<usize>, number: T, f: F) -> Option<usize> {
	match on.end.checked_sub(on.start)? {
		0 => None,
		1 => Some(on.start).filter(|&x| f(x) == number),
		_ => {
			let left = f(on.start);
			let right = f(on.end-1);
			if left > right {
				binary_search(on, |pos| f(pos) <= number).filter(|x| f(*x) == number)
			} else if left < right {
				binary_search(on, |pos| f(pos) >= number).filter(|x| f(*x) == number)
			} else if left == number {
				Some(on.start)
			} else {
				None
			}
		}
	}
}

// Finds range in `on` where all numbers are equals to `number`. `f` must be monotone function.
fn binary_search_number_range<F: Fn(usize) -> T, T: Ord + Eq + Copy + Debug>(on: Range<usize>, number: T, f: F) -> Option<Range<usize>> {
	let left = binary_search_number_leftmost(on.clone(), number, &f)?;
	let right = binary_search(left..on.end, |pos| f(pos) != number).unwrap_or(on.end);
	Some(left..right)
}

#[cfg(test)]
mod tests_binary_search {
	use super::*;

	#[test]
	fn testing() {
		fn test(vec: Vec<bool>) -> Option<usize> {
			binary_search(0..vec.len(), |pos| vec[pos])
		}
		assert_eq!(test(vec![]), None);
		assert_eq!(test(vec![false]), None);
		assert_eq!(test(vec![true]), Some(0));
		assert_eq!(test(vec![false, false]), None);
		assert_eq!(test(vec![false, true]), Some(1));
		assert_eq!(test(vec![true, true]), Some(0));
		assert_eq!(test(vec![false, false, true]), Some(2));
		assert_eq!(test(vec![false, true, true]), Some(1));
		assert_eq!(test(vec![true, true, true]), Some(0));
		assert_eq!(test(vec![false, false, false, false]), None);
		assert_eq!(test(vec![false, false, false, true]), Some(3));
		assert_eq!(test(vec![false, false, true, true]), Some(2));
		assert_eq!(test(vec![false, true, true, true]), Some(1));
		assert_eq!(test(vec![true, true, true, true]), Some(0));
		assert_eq!(test(vec![false, false, false, false, false]), None);
		assert_eq!(test(vec![false, false, false, false, true]), Some(4));
		assert_eq!(test(vec![false, false, false, true, true]), Some(3));
		assert_eq!(test(vec![false, false, true, true, true]), Some(2));
		assert_eq!(test(vec![false, true, true, true, true]), Some(1));
		assert_eq!(test(vec![true, true, true, true, true]), Some(0));
	}
}

#[cfg(test)]
mod tests_binary_search_number_range {
	use super::*;

	#[test]
	fn testing() {
		fn test_number(arr: &[u32], number: u32) {
			let result = binary_search_number_leftmost(0..arr.len(), number, |x| arr[x]);
			let start = arr.iter().enumerate().find(|(_, value)| **value == number).map(|(index, _)| index);
			if result != start {
				dbg!(arr, result, start);
				panic!();
			}
		}
		fn test_range(arr: &[u32], number: u32) {
			let result1 = binary_search_number_range(0..arr.len(), number, |x| arr[x]);
			let start = arr.iter().enumerate().find(|(_, value)| **value == number).map(|(index, _)| index);
			let end = arr.iter().enumerate().rev().find(|(_, value)| **value == number).map(|(index, _)| index);
			let result2 = start.zip(end).map(|(a, b)| a..b+1);
			if result1 != result2 {
				dbg!(arr, result1, result2);
				panic!();
			}
		}
		use rand::Rng;
		let mut rng = rand::thread_rng();
		for _ in 0..200000 {
			let mut arr = (0..rng.gen_range(0, 20)).map(|_| rng.gen_range(0, 10)).collect::<Vec<u32>>();
			arr.sort_unstable();
			if rng.gen() {
				arr.reverse();
			}
			let number = rng.gen_range(0, 10);
			test_number(&arr, number);
			test_range(&arr, number);
		}
	}
}
