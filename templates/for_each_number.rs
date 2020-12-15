fn for_each_number<F: FnMut(&[u8])>(base: u8, size: usize, mut f: F) {
    fn helper<F: FnMut(&[u8])>(index: usize, base: u8, number: &mut [u8], f: &mut F) {
        if index == number.len() {
            f(&number);
        } else {
            for i in 0..base {
                number[index] = i;
                helper(index + 1, base, number, f);
            }
        }
    }

    let mut number = vec![0u8; size];
    helper(0, base, &mut number, &mut f);
}

#[derive(Clone, Debug)]
struct SubsetIter<'a, 'b, T> {
    can: &'a [u8],
    set: &'b [T],
    pos: usize,
}

impl<'b, T> Iterator for SubsetIter<'_, 'b, T> {
    type Item = &'b T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let len = self.can.len();
            while self.pos < len {
                if *self.can.get_unchecked(self.pos) == 1 {
                    let result = Some(self.set.get_unchecked(self.pos));
                    self.pos += 1;
                    return result;
                }
                self.pos += 1;
            }
            None
        }
    }
}

fn for_each_subset<'b, T, F: FnMut(SubsetIter<'_, 'b, T>)>(set: &'b [T], mut f: F) {
    for_each_number(2, set.len(), |can| {
        f(SubsetIter { can, set, pos: 0 });
    })
}
