use std::io::BufRead;

fn get_min_divisor(n: i64) -> Option<i64> {
	for i in (2..=((n as f64).sqrt() as i64 + 5)).filter(|x| *x < n) {
		if n % i == 0 {
			return Some(i);
		}
	}
	None
}

fn get_answer(n: i64, depth: i64) -> i64 {
	match n {
		1 => 0,
		2 => 1,
		3 => 2,
		n => match get_min_divisor(n) {
			Some(d) => {
				1 + if depth < 4 {
					std::cmp::min(get_answer(d, depth + 1), get_answer(n - 1, depth + 1))
				} else {
					get_answer(d, depth)
				}
			},
			None => 1 + get_answer(n - 1, depth),
		},
	}
}

use std::str::FromStr;

fn read<T: FromStr, I: Iterator<Item = std::io::Result<String>>>(i: &mut I) -> T
where <T as std::str::FromStr>::Err: std::fmt::Debug {
	i.next().unwrap().unwrap().parse().unwrap()
}

pub fn main() {
	let input = std::io::stdin();
	let mut stdin = input.lock().lines();
	let count: i64 = read(&mut stdin);
	for _ in 0..count {
		let n: i64 = read(&mut stdin);
		println!("{}", get_answer(n, 0));
	}
}
