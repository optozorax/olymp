// Division with rounding up. ceil_div(2, 3) = 1, ceil_div(5, 2) = 3
fn ceil_div<T>(x: T, y: T) -> T where T: Copy + Rem<Output = T> + Div<Output = T> + Add<Output = T> + From<u8> + PartialEq {
	x / y + if x % y == T::from(0u8) { T::from(0) } else { T::from(1) }
}
