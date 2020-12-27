struct Suffix<'a, T, F> {
    on: &'a [T],
    inner: Vec<T>,
    f: F,
}

impl<'a, T: Clone, F: Fn(T, T) -> T> Suffix<'a, T, F> {
    fn new(on: &'a [T], f: F) -> Self {
        let mut state = None;
        let mut inner = on
            .iter()
            .rev()
            .map(|x| {
                if let Some(st) = state.clone() {
                    let result = f(x.clone(), st);
                    state = Some(result.clone());
                    result
                } else {
                    let result = x.clone();
                    state = Some(result.clone());
                    result
                }
            })
            .collect::<Vec<_>>();
        if let Some(ref st) = state {
            inner.push(st.clone());
        }
        Self { on, inner, f }
    }

    // Same as run function `f` on `on[pos..]`
    fn get_suffix_f(&self, pos: usize) -> Option<T> {
        if pos == self.on.len() {
            return None;
        }
        self.inner.get(self.on.len() - (pos + 1)).cloned()
    }

    // This works only for inversible operations, like `xor`, `+` etc.
    // For f = |x, y| x+y, f_invert = |x, y| x-y
    // For f = |x, y| x^y, f_invert = |x, y| x^y
    // f(a, b) == f_invert(f(f(a, b), c), c)
    fn get_segment_f<F1: Fn(T, T) -> T>(&self, range: Range<usize>, f_invert: F1) -> Option<T> {
        let start = self.get_suffix_f(range.start)?;
        if let Some(end) = self.get_suffix_f(range.end) {
            Some(f_invert(start, end))
        } else {
            Some(start)
        }
    }
}
