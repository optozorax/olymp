// Returned value `x` which minimizes `|x0-x|+|x1-x|+...+|xn-x|`
fn median<T: Ord + Clone>(x: &mut [T]) -> T {
    assert!(!x.is_empty());
    x.sort_unstable();
    x[x.len() / 2].clone()
}

// Allocating version of function `median`
fn median_clone<T: Ord + Clone>(x: &[T]) -> T {
    let mut x = x.to_vec();
    median(&mut x)
}
