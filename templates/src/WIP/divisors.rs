// TODO refactor and improve this thing
fn get_min_divisor(n: i64) -> Option<i64> {
	for i in (2..=((n as f64).sqrt() as i64 + 5)).filter(|x| *x < n) {
		if n % i == 0 {
			return Some(i);
		}
	}
	None
}
