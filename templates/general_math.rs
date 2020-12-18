fn fastmod<T: Ord + Copy + Sub<Output = T>>(x: T, y: T) -> T {
    if x > y {
        x - y
    } else {
        x
    }
}