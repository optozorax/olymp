struct SegmentTree<'a, T, F> {
	input: &'a [T],
	tree: Vec<T>,
	f: F,
	neutral: T,
}

impl<'a, T: Clone + Default, F: Fn(T, T) -> T> SegmentTree<'a, T, F> {
	pub fn create(input: &'a [T], f: F, neutral: T) -> SegmentTree<'a, T, F> {
		let tree = vec![T::default(); input.len().next_power_of_two() * 2];
		let mut result = Self {
			input,
			tree,
			f,
			neutral,
		};
		result.build(1, 0, input.len().next_power_of_two()-1);
		result
	}

	fn build(&mut self, pos: usize, l: usize, r: usize) {
		if l == r {
			self.tree[pos] = self.input.get(l).cloned().unwrap_or_else(|| self.neutral.clone());
		} else {
			let c = (l+r)/2;
			self.build(Self::child1(pos), l, c);
			self.build(Self::child2(pos), c+1, r);
			self.tree[pos] = (self.f)(self.tree[Self::child1(pos)].clone(), self.tree[Self::child2(pos)].clone());
		}
	}

	fn child1(pos: usize) -> usize {
		pos * 2
	}

	fn child2(pos: usize) -> usize {
		pos * 2 + 1
	}

	fn f_for_segment_helper(&self, pos: usize, l: usize, r: usize, from: usize, to: usize) -> Option<T> {
		if from > r || to < l {
			None
		} else if l <= from && to <= r {
			Some(self.tree.get(pos).cloned().unwrap_or_else(|| self.input[pos-self.tree.len()].clone()))
		} else {
			let c = (from + to)/2;
			let left = self.f_for_segment_helper(Self::child1(pos), l, r, from, c);
			let right = self.f_for_segment_helper(Self::child2(pos), l, r, c+1, to);
			match (left, right) {
				(Some(a), Some(b)) => Some((self.f)(a, b)),
				(Some(a), None) => Some(a),
				(None, Some(b)) => Some(b),
				(None, None) => None,
			}
		}
	}

	pub fn f_for_segment(&self, l: usize, r: usize) -> T {
		self.f_for_segment_helper(1, l, r, 0, self.input.len().next_power_of_two()-1).unwrap()
	}
}

#[cfg(test)]
mod segment_tree_tests {
	use super::*;

	#[test]
	fn segment_tree() {
		fn test(vec: Vec<u32>) {
			let tree = SegmentTree::create(&vec, std::cmp::min, u32::MAX);
			for i in 0..vec.len() {
				for j in i+1..vec.len() {
					dbg!(i, j);
					assert_eq!(tree.f_for_segment(i, j), vec[i..=j].iter().copied().min().unwrap());
				}
			}	
		}
		test(vec![]);
		test(vec![1]);
		test(vec![1, 2]);
		test(vec![1, 3, 4]);
		test(vec![1, 3, 4, 5]);
		test(vec![1, 2, 3, 3, 3, 4, 4, 3, 4, 2, 1]);
		
	}
}