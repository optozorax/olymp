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
