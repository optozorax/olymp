struct ScanBeforeFirst<I, T, F> {
	iter: I,
	value: T,
	f: F,
}

trait ScanBeforeFirstTrait<T>: Sized + Iterator<Item = T> {
	fn scan_before_first<F: Fn(T, &T) -> T>(self, f: F) -> ScanBeforeFirst<Self, T, F>;
}

impl<T: Default, I: Sized + Iterator<Item = T>> ScanBeforeFirstTrait<T> for I {
	fn scan_before_first<F: Fn(T, &T) -> T>(mut self, f: F) -> ScanBeforeFirst<Self, T, F> {
		let value = self.next().unwrap_or_default();
		ScanBeforeFirst { iter: self, value, f }
	}
}

impl<I, T, F> Iterator for ScanBeforeFirst<I, T, F>
where
	F: Fn(T, &T) -> T,
	I: Sized + Iterator<Item = T>,
	T: Clone,
{
	type Item = (T, T);

	fn next(&mut self) -> Option<Self::Item> {
		let next_t = self.iter.next()?;
		let state = self.value.clone();
		self.value = (self.f)(state, &next_t);
		Some((self.value.clone(), next_t))
	}
}

impl<I, T, F> ExactSizeIterator for ScanBeforeFirst<I, T, F>
where
	F: Fn(T, &T) -> T,
	I: Sized + Iterator<Item = T> + ExactSizeIterator,
	T: Clone,
{
	fn len(&self) -> usize { self.iter.len() }
}

impl<I, T, F> FusedIterator for ScanBeforeFirst<I, T, F>
where
	F: Fn(T, &T) -> T,
	I: Sized + Iterator<Item = T> + FusedIterator,
	T: Clone,
{
}