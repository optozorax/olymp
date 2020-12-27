/// https://cp-algorithms.com/data_structures/sparse-table.html
/// Can be applied to all idempotent function. Idempotent function is f(x, x) = x. Examples: gcd, min, max.
#[derive(Debug)]
struct SparseTable<'a, B: BinaryFunction> {
	input: &'a [B::T],
	inner: Vec<Vec<B::Y>>,
	b: B,
}

impl<'a, B: BinaryFunction> SparseTable<'a, B> {
	pub fn create(input: &'a [B::T], b: B) -> Self {
		let levels = Self::largest_bit(input.len());
		let mut inner = Vec::with_capacity(levels);
		inner.push(
			(0..input.len())
				.map(|pos| b.up(pos, input[pos].clone()))
				.collect::<Vec<B::Y>>(),
		);
		let mut result = Self { input, inner, b };
		for k in 1..=levels {
			let previous = result.inner.last().unwrap();
			let to_push = (0..(input.len() - (1 << k) + 1))
				.map(|i| result.b.f(previous[i].clone(), previous[i + (1 << (k - 1))].clone()))
				.collect::<Vec<_>>();
			result.inner.push(to_push);
		}
		result
	}

	fn largest_bit(a: usize) -> usize {
		(std::mem::size_of::<usize>() * 8 - 1).saturating_sub(a.leading_zeros() as usize)
	}

	pub fn f_for_segment(&self, on: Range<usize>) -> B::Y {
		assert!(on.start < on.end);
		let a = on.start;
		let b = on.end;
		let level = Self::largest_bit(b - a);
		self.b
			.f(self.inner[level][a].clone(), self.inner[level][b - (1 << level)].clone())
	}
}
