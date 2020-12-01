struct Prefix<'a, T, F> {
    on: &'a [T],
    inner: Vec<T>,
    f: F,
}

impl<'a, T: Clone, F: Fn(T, T) -> T> Prefix<'a, T, F> {
    fn new(on: &'a [T], f: F) -> Self {
        let mut state = None;
        let mut inner = on
            .iter()
            .map(|x| {
                if let Some(st) = state.clone() {
                    let result = f(st, x.clone());
                    state = Some(result.clone());
                    result
                } else {
                    let result = x.clone();
                    state = Some(result.clone());
                    result
                }
            })
            .collect::<Vec<_>>();
        if let Some(st) = state {
            inner.push(st);
        }
        Self { on, inner, f }
    }

    fn get_prefix_f(&self, pos: usize) -> Option<T> {
        if pos == 0 {
            return None;
        }
        self.inner.get(pos - 1).cloned()
    }

    // This works only for inversible operations, like `xor`, `+` etc.
    // For f = |x, y| x+y, f_invert = |x, y| x-y
    // For f = |x, y| x^y, f_invert = |x, y| x^y
    // f(a, b) == f_invert(f(f(a, b), c), c)
    fn get_segment_f<F1: Fn(T, T) -> T>(&self, range: Range<usize>, f_invert: F1) -> Option<T> {
        let end = self.get_prefix_f(range.end)?;
        if let Some(start) = self.get_prefix_f(range.start) {
            Some(f_invert(end, start))
        } else {
            Some(end)
        }
    }
}

#[cfg(test)]
mod prefix_tests {
    use super::*;

    #[test]
    fn sum() {
        let a: Vec<i32> = vec![1, 2, -3, 0, 555, -1, 0, 0];
        let pr_sum = Prefix::new(&a, |x, y| x + y);
        assert_eq!(pr_sum.get_prefix_f(0), None);
        for i in 1..=a.len() {
            assert_eq!(pr_sum.get_prefix_f(i), Some(a[0..i].iter().sum::<i32>()));
        }

        for i in 0..a.len() {
            for j in i + 1..=a.len() {
                assert_eq!(
                    pr_sum.get_segment_f(i..j, |x, y| x - y),
                    Some(a[i..j].iter().sum::<i32>())
                );
            }
        }
    }

    #[test]
    fn xor() {
        let a: Vec<i32> = vec![1, 2, 32, 6, 7, 8, 53, 33];
        let pr_sum = Prefix::new(&a, |x, y| x ^ y);
        assert_eq!(pr_sum.get_prefix_f(0), None);
        for i in 1..=a.len() {
            assert_eq!(
                pr_sum.get_prefix_f(i),
                Some(a[0..i].iter().fold(1, |acc, x| acc ^ x) ^ 1)
            );
        }

        for i in 0..a.len() {
            for j in i + 1..=a.len() {
                assert_eq!(
                    pr_sum.get_segment_f(i..j, |x, y| x ^ y),
                    Some(a[i..j].iter().fold(1, |acc, x| acc ^ x) ^ 1)
                );
            }
        }
    }

    #[test]
    fn min() {
        let a: Vec<i32> = vec![1, 2, -3, 0, 555, -1, 0, 0];
        let pr_sum = Prefix::new(&a, std::cmp::min);
        assert_eq!(pr_sum.get_prefix_f(0), None);
        for i in 1..=a.len() {
            assert_eq!(pr_sum.get_prefix_f(i), a[0..i].iter().copied().min());
        }
    }
}