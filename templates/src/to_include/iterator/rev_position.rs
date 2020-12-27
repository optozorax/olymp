trait RevPositionTrait: Iterator + DoubleEndedIterator {
	fn rev_position<P: FnMut(Self::Item) -> bool>(&mut self, predicate: P) -> Option<usize>;
}

impl<I: Iterator + DoubleEndedIterator + ExactSizeIterator> RevPositionTrait for I {
	fn rev_position<P: FnMut(Self::Item) -> bool>(&mut self, predicate: P) -> Option<usize> {
		Some(self.len() - 1 - self.rev().position(predicate)?)
	}
}
