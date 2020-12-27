trait BinaryFunction {
	type T: Clone;
	type Y: Clone;

	fn up(&self, pos: usize, t: Self::T) -> Self::Y;
	fn f(&self, a: Self::Y, b: Self::Y) -> Self::Y;
}

impl<T, Y, F1, F2> BinaryFunction for (F1, F2, PhantomData<T>, PhantomData<Y>)
where
	T: Clone,
	Y: Clone,
	F1: Fn(usize, T) -> Y,
	F2: Fn(Y, Y) -> Y,
{
	type T = T;
	type Y = Y;

	fn up(&self, pos: usize, t: Self::T) -> Self::Y { (self.0)(pos, t) }

	fn f(&self, a: Self::Y, b: Self::Y) -> Self::Y { (self.1)(a, b) }
}
