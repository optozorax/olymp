fn fastmod<T: Ord + Copy + Sub<Output = T>>(x: T, y: T) -> T {
    if x > y {
        x - y
    } else {
        x
    }
}

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

// .fold(1, |acc, x| gcd(acc, x))
fn lcm<T>(a: T, b: T) -> T
where
    T: PartialOrd
        + PartialEq
        + Rem<Output = T>
        + From<u8>
        + Copy
        + Mul<Output = T>
        + Neg<Output = T>
        + Div<Output = T>,
{
    let mul = a * b;
    let mul = if mul < T::from(0) { mul } else { -mul };
    mul / gcd(a, b)
}
