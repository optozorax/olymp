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
