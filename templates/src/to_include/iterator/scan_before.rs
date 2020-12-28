struct ScanBefore<I, S, F> {
	iter: I,
	value: S,
	f: F,
}

trait ScanBeforeTrait<T>: Sized + Iterator<Item = T> {
	fn scan_before<S, F: Fn(S, &T) -> S>(self, init: S, f: F) -> ScanBefore<Self, S, F>;
}

impl<T, I: Sized + Iterator<Item = T>> ScanBeforeTrait<T> for I {
	fn scan_before<S, F: Fn(S, &T) -> S>(self, value: S, f: F) -> ScanBefore<Self, S, F> {
		ScanBefore { iter: self, value, f }
	}
}

impl<I, T, S, F> Iterator for ScanBefore<I, S, F>
where
	F: Fn(S, &T) -> S,
	I: Sized + Iterator<Item = T>,
	S: Clone,
{
	type Item = (S, T);

	fn next(&mut self) -> Option<Self::Item> {
		let next_t = self.iter.next()?;
		let state = self.value.clone();
		self.value = (self.f)(state, &next_t);
		Some((self.value.clone(), next_t))
	}
}

impl<I, T, S, F> ExactSizeIterator for ScanBefore<I, S, F>
where
	F: Fn(S, &T) -> S,
	I: Sized + Iterator<Item = T> + ExactSizeIterator,
	S: Clone,
{
	fn len(&self) -> usize { self.iter.len() }
}

impl<I, T, S, F> FusedIterator for ScanBefore<I, S, F>
where
	F: Fn(S, &T) -> S,
	I: Sized + Iterator<Item = T> + FusedIterator,
	S: Clone,
{
}
