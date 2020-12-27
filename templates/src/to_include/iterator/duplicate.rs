trait FindDuplicatesWithVecBy: Sized {
	fn duplicates_with_vec_by<T: Clone, F: Fn(T) -> usize>(self, vec_size: usize, f: F) -> DuplicatesWithVecBy<Self, F, T> where Self: std::iter::Iterator<Item = T>;
}

impl<I: Iterator> FindDuplicatesWithVecBy for I {
	fn duplicates_with_vec_by<T: Clone, F: Fn(T) -> usize>(self, vec_size: usize, f: F) -> DuplicatesWithVecBy<Self, F, T> where Self: std::iter::Iterator<Item = T> {
		DuplicatesWithVecBy {
			vec: vec![None; vec_size],
			iter: self,
			f,
		}
	}
}

struct DuplicatesWithVecBy<I: Iterator<Item = T>, F: Fn(T) -> usize, T: Clone> {
	vec: Vec<Option<T>>,
	iter: I,
	f: F,
}

impl<I: Iterator<Item = T>, F: Fn(T) -> usize, T: Clone> Iterator for DuplicatesWithVecBy<I, F, T> {
	type Item = (T, T);

	fn next(&mut self) -> Option<Self::Item> {
		while let Some(elem) = self.iter.next() {
			let pos = (self.f)(elem.clone());
			dbg!(pos);
			if let Some(exists) = &self.vec[pos] {
				return Some((exists.clone(), elem))
			} else {
				self.vec[pos] = Some(elem);
			}
		}
		None
	}
}
