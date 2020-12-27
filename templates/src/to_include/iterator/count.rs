trait CountVecTrait {
	fn collect_count(self, max: usize) -> Vec<usize>;
}

impl<T: Iterator<Item = K>, K: Into<usize>> CountVecTrait for T {
	fn collect_count(self, max: usize) -> Vec<usize> {
		let mut result = vec![0; max];
		for i in self {
			result[i.into()] += 1;
		}
		result
	}	
}
