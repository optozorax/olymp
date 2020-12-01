// Iterates over all permutation of array
// Make it iterator is impossible because of lack of GAT and therefore StreamingIterator
fn permutations<T, F: FnMut(&[T])>(a: &mut [T], mut f: F) {
    fn helper<T, F: FnMut(&[T])>(k: usize, a: &mut [T], f: &mut F) {
        if k == 1 {
            f(a);
        } else {
            helper(k - 1, a, f);
            for i in 0..k - 1 {
                if k % 2 == 0 {
                    a.swap(i, k - 1);
                } else {
                    a.swap(0, k - 1);
                }
                helper(k - 1, a, f);
            }
        }
    }
    helper(a.len(), a, &mut f);
}
