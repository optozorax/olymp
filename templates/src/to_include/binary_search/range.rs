// Finds range in `on` where all numbers are equals to `number`. `f` must be monotone function.
fn binary_search_number_range<F: Fn(usize) -> T, T: Ord + Eq + Copy + Debug>(on: Range<usize>, number: T, f: F) -> Option<Range<usize>> {
	let left = binary_search_number_leftmost(on.clone(), number, &f)?;
	let right = binary_search(left..on.end, |pos| f(pos) != number).unwrap_or(on.end);
	Some(left..right)
}
