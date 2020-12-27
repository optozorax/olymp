#[derive(Debug)]
struct MinOrMax<T> {
	kind: RmqType,
	phantom: PhantomData<T>,
}

impl<T: Clone + Ord> BinaryFunction for MinOrMax<T> {
	type T = T;
	type Y = (usize, T);

	fn up(&self, pos: usize, t: Self::T) -> Self::Y { (pos, t) }

	fn f(&self, a: Self::Y, b: Self::Y) -> Self::Y {
		match self.kind {
			RmqType::Min => {
				if a.1 < b.1 {
					a
				} else {
					b
				}
			},
			RmqType::Max => {
				if a.1 > b.1 {
					a
				} else {
					b
				}
			},
		}
	}
}

#[derive(Debug)]
struct Rmq<'a, T: Clone + Ord> {
	sparse_table: SparseTable<'a, MinOrMax<T>>,
}

#[derive(Clone, Debug)]
enum RmqType {
	Min,
	Max,
}

impl<'a, T: Ord + Copy> Rmq<'a, T> {
	pub fn create(input: &'a [T], kind: RmqType) -> Self {
		let binary_function = MinOrMax { kind, phantom: PhantomData };
		let sparse_table = SparseTable::create(input, binary_function);
		Self { sparse_table }
	}

	pub fn most_index_for_segment(&self, on: Range<usize>) -> usize { self.sparse_table.f_for_segment(on).0 }

	pub fn most_for_segment(&self, on: Range<usize>) -> T {
		assert!(on.start != on.end);
		self.sparse_table.f_for_segment(on).1
	}
}
