fn for_each_subslice<T, F: FnMut(&[T])>(input: &[T], mut f: F) {
	for i in 0..input.len() {
		for j in i + 1..=input.len() {
			f(&input[i..j]);
		}
	}
}