// .fold(0, |acc, x| gcd(acc, x))
fn gcd<T>(a1: T, a2: T) -> T
where
    T: PartialOrd + PartialEq + Rem<Output = T> + From<u8> + Copy,
{
    // variable names based off Euclidean divison equation: a = b · q + r
    let (mut a, mut b) = if a1 > a2 { (a1, a2) } else { (a2, a1) };

    while b != T::from(0) {
        let r = a % b;
        a = b;
        b = r;
    }

    a
}
