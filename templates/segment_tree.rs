#[derive(Debug)]
struct SegmentTree<'a, T, Y, F, FY> {
    input: &'a mut [T],
    tree: Vec<Option<Y>>,
    f: F,
    fy: FY,
}

impl<'a, T: Clone, Y: Clone, F: Fn(Range<usize>, Y, Y) -> Y, FY: Fn(usize, T) -> Y>
    SegmentTree<'a, T, Y, F, FY>
{
    fn child1(pos: usize) -> usize {
        pos * 2
    }

    fn child1range(on: Range<usize>) -> Range<usize> {
        let c = (on.start + (on.end - 1)) / 2;
        on.start..c + 1
    }

    fn child2(pos: usize) -> usize {
        pos * 2 + 1
    }

    fn child2range(on: Range<usize>) -> Range<usize> {
        let c = (on.start + (on.end - 1)) / 2;
        c + 1..on.end
    }

    fn start_range(&self) -> Range<usize> {
        0..self.input.len().next_power_of_two()
    }

    fn build(&mut self, pos: usize, on: Range<usize>) -> Option<Y> {
        let result = if on.end - on.start == 2 {
            let a = self
                .input
                .get(on.start)
                .cloned()
                .map(|x| (self.fy)(on.start, x));
            let b = self
                .input
                .get(on.end - 1)
                .cloned()
                .map(|x| (self.fy)(on.end - 1, x));
            a.any_or_both_with(b, |x, y| (self.f)(on, x, y))
        } else {
            let a = self.build(Self::child1(pos), Self::child1range(on.clone()));
            let b = self.build(Self::child2(pos), Self::child2range(on.clone()));
            a.any_or_both_with(b, |x, y| (self.f)(on, x, y))
        };
        self.tree[pos] = result.clone();
        result
    }

    fn f_for_segment_helper(
        &self,
        pos: usize,
        on: Range<usize>,
        current: Range<usize>,
    ) -> Option<Y> {
        if current.start >= on.end || current.end <= on.start {
            None
        } else if on.start <= current.start && current.end <= on.end {
            self.tree.get(pos).cloned().unwrap_or_else(|| {
                Some((self.fy)(
                    pos - self.tree.len(),
                    self.input.get(pos - self.tree.len())?.clone(),
                ))
            })
        } else {
            let a = self.f_for_segment_helper(
                Self::child1(pos),
                on.clone(),
                Self::child1range(current.clone()),
            );
            let b = self.f_for_segment_helper(
                Self::child2(pos),
                on.clone(),
                Self::child2range(current),
            );
            a.any_or_both_with(b, |x, y| (self.f)(on, x, y))
        }
    }

    #[allow(clippy::collapsible_if)]
    fn set_helper(&mut self, pos: usize, index: usize, current: Range<usize>, t: T) -> Option<Y> {
        let (a, b) = if current.end - current.start == 2 {
            self.input[index] = t;
            (
                self.input
                    .get(current.start)
                    .cloned()
                    .map(|x| (self.fy)(current.start, x)),
                self.input
                    .get(current.end - 1)
                    .cloned()
                    .map(|x| (self.fy)(current.end - 1, x)),
            )
        } else {
            if Self::child1range(current.clone()).contains(&index) {
                (
                    self.set_helper(
                        Self::child1(pos),
                        index,
                        Self::child1range(current.clone()),
                        t,
                    ),
                    self.tree[Self::child2(pos)].clone(),
                )
            } else {
                (
                    self.tree[Self::child1(pos)].clone(),
                    self.set_helper(
                        Self::child2(pos),
                        index,
                        Self::child2range(current.clone()),
                        t,
                    ),
                )
            }
        };
        self.tree[pos] = a.any_or_both_with(b, |x, y| (self.f)(current, x, y));
        self.tree[pos].clone()
    }

    pub fn create(input: &'a mut [T], f: F, fy: FY) -> SegmentTree<'a, T, Y, F, FY> {
        let should_size = input.len().next_power_of_two();
        let tree = vec![None; should_size];
        let mut result = Self { input, tree, f, fy };
        if should_size > 1 {
            result.build(1, result.start_range());
        }
        result
    }

    pub fn f_for_segment(&self, on: Range<usize>) -> Option<Y> {
        self.f_for_segment_helper(1, on, self.start_range())
    }

    // Returns the array *on* which this tree is created
    pub fn on(&'a self) -> &'a [T] {
        self.input
    }

    pub fn set(&mut self, index: usize, t: T) {
        assert!(index < self.input.len());
        if self.tree.len() != 1 {
            self.set_helper(1, index, self.start_range(), t);
        } else {
            self.input[index] = t;
        }
    }
}

#[cfg(test)]
mod segment_tree_tests {
    use super::*;

    #[test]
    fn segment_tree() {
        color_backtrace::install();
        fn test(vec: Vec<u64>) {
            use std::ops::*;
            test1(vec.clone(), std::cmp::min);
            test1(vec.clone(), std::cmp::max);
            test1(vec.clone(), u64::add);
            test1(vec.clone(), u64::bitxor);
            test1(vec.clone(), u64::mul);
            drop(vec);
        }
        fn test1<F: Fn(u64, u64) -> u64 + Copy>(mut vec: Vec<u64>, operation: F) {
            let mut vec_clone = vec.clone();
            use rand::Rng;
            let mut rng = rand::thread_rng();
            let mut tree = SegmentTree::create(&mut vec, |_, x, y| operation(x, y), |_, x| x);
            for _ in 0..200 {
                if !tree.on().is_empty() {
                    let pos = rng.gen_range(0, tree.on().len());
                    let value = rng.gen_range(0, 50);
                    tree.set(pos, value);
                    vec_clone[pos] = value;
                }
                for i in 0..tree.on().len() {
                    for j in i + 1..=tree.on().len() {
                        let fast = tree.f_for_segment(i..j);
                        let long = Some(
                            vec_clone[i + 1..j]
                                .iter()
                                .copied()
                                .fold(vec_clone[i], operation),
                        )
                        .filter(|_| j - i > 0);
                        if fast != long {
                            dbg!(i..j, tree.tree, tree.input, vec_clone, fast, long);
                            panic!();
                        }
                    }
                }
            }
        }
        test(vec![]);
        test(vec![1]);
        test(vec![1, 2]);
        test(vec![1, 3, 4]);
        test(vec![1, 3, 4, 5]);
        test(vec![1, 2, 3, 3, 3, 4, 4, 3, 4, 2, 1]);
        test(vec![1, 2, 3, 0, 555, 1, 0, 0]);
        test(vec![1, 2, 32, 6, 7, 8, 53, 33]);
    }
}