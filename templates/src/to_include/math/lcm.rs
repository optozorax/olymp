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
