struct ScanAfterFirst<I, T, F> {
	iter: I,
	value: T,
	f: F,
}

trait ScanAfterFirstTrait<T>: Sized + Iterator<Item = T> {
	fn scan_after_first<F: Fn(T, &T) -> T>(self, f: F) -> ScanAfterFirst<Self, T, F>;
}

impl<T: Default, I: Sized + Iterator<Item = T>> ScanAfterFirstTrait<T> for I {
	fn scan_after_first<F: Fn(T, &T) -> T>(mut self, f: F) -> ScanAfterFirst<Self, T, F> {
		let value = self.next().unwrap_or_default();
		ScanAfterFirst { iter: self, value, f }
	}
}

impl<I, T, F> Iterator for ScanAfterFirst<I, T, F>
where
	F: Fn(T, &T) -> T,
	I: Sized + Iterator<Item = T>,
	T: Clone,
{
	type Item = (T, T);

	fn next(&mut self) -> Option<Self::Item> {
		let next_t = self.iter.next()?;
		let state = self.value.clone();
		self.value = (self.f)(self.value.clone(), &next_t);
		Some((state, next_t))
	}
}

impl<I, T, F> ExactSizeIterator for ScanAfterFirst<I, T, F>
where
	F: Fn(T, &T) -> T,
	I: Sized + Iterator<Item = T> + ExactSizeIterator,
	T: Clone,
{
	fn len(&self) -> usize { self.iter.len() }
}

impl<I, T, F> FusedIterator for ScanAfterFirst<I, T, F>
where
	F: Fn(T, &T) -> T,
	I: Sized + Iterator<Item = T> + FusedIterator,
	T: Clone,
{
}