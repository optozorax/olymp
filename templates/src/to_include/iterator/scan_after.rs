struct ScanAfter<I, S, F> {
	iter: I,
	value: S,
	f: F,
}

trait ScanAfterTrait<T>: Sized + Iterator<Item = T> {
	fn scan_after<S, F: Fn(S, &T) -> S>(self, init: S, f: F) -> ScanAfter<Self, S, F>;
}

impl<T, I: Sized + Iterator<Item = T>> ScanAfterTrait<T> for I {
	fn scan_after<S, F: Fn(S, &T) -> S>(self, value: S, f: F) -> ScanAfter<Self, S, F> {
		ScanAfter { iter: self, value, f }
	}
}

impl<I, T, S, F> Iterator for ScanAfter<I, S, F>
where
	F: Fn(S, &T) -> S,
	I: Sized + Iterator<Item = T>,
	S: Clone,
{
	type Item = (S, T);

	fn next(&mut self) -> Option<Self::Item> {
		let next_t = self.iter.next()?;
		let state = self.value.clone();
		self.value = (self.f)(self.value.clone(), &next_t);
		Some((state, next_t))
	}
}

impl<I, T, S, F> ExactSizeIterator for ScanAfter<I, S, F>
where
	F: Fn(S, &T) -> S,
	I: Sized + Iterator<Item = T> + ExactSizeIterator,
	S: Clone,
{
	fn len(&self) -> usize { self.iter.len() }
}

impl<I, T, S, F> FusedIterator for ScanAfter<I, S, F>
where
	F: Fn(S, &T) -> S,
	I: Sized + Iterator<Item = T> + FusedIterator,
	S: Clone,
{
}
