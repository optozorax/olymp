// TODO refactor and improve this thing
fn get_min_divisor(n: i64) -> Option<i64> {
	for i in (2..=((n as f64).sqrt() as i64 + 5)).filter(|x| *x < n) {
		if n % i == 0 {
			return Some(i);
		}
	}
	None
}

fn factorize(mut n: u64) -> BTreeMap<u64, usize> {
	let mut result = BTreeMap::new();
	for i in 2..n {
		if i > n || i > (n as f64).sqrt() as u64 + 5 {
			break;
		}
		while n % i == 0 {
			*result.entry(i).or_insert(0) += 1;
			n /= i;
		}
	}
	if n != 0 {
		*result.entry(n).or_insert(0) += 1;
	}
	result
}
