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

    // Same as run function `f` on `on[0..pos]`
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
