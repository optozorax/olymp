use std::{
	io::{stdout, BufRead, BufWriter, Write},
	str::FromStr,
};

fn read<T: FromStr, I: Iterator<Item = std::io::Result<String>>>(i: &mut I) -> T
where <T as std::str::FromStr>::Err: std::fmt::Debug {
	i.next().unwrap().unwrap().parse().unwrap()
}

fn moves_up(token: (u64, u64), d: u64, k: u64) -> Option<u64> {
	let result = ((d as f64 * d as f64 - token.0 as f64 * token.0 as f64).sqrt().floor() as u64 - token.1) / k;
	if result == 0 { None } else { Some(result) }
}

fn moves_right(token: (u64, u64), d: u64, k: u64) -> Option<u64> {
	let result = ((d as f64 * d as f64 - token.1 as f64 * token.1 as f64).sqrt().floor() as u64 - token.0) / k;
	if result == 0 { None } else { Some(result) }
}

fn is_utkarsh_wins(d: u64, k: u64) -> bool {
	let mut token = (0, 0);
	loop {
		// Ashish
		match (moves_up(token, d, k), moves_right(token, d, k)) {
			(Some(up), Some(right)) => {
				if up > right {
					token.1 += k
				} else {
					token.0 += k
				}
			},
			(Some(_), None) => token.1 += k,
			(None, Some(_)) => token.0 += k,
			(None, None) => return true,
		}

		// Utkarish
		match (moves_up(token, d, k), moves_right(token, d, k)) {
			(Some(up), Some(right)) => {
				if up > right {
					token.1 += k
				} else {
					token.0 += k
				}
			},
			(Some(_), None) => token.1 += k,
			(None, Some(_)) => token.0 += k,
			(None, None) => return false,
		}
	}
}

fn read_vec<T: FromStr, I: Iterator<Item = std::io::Result<String>>>(i: &mut I) -> Vec<T>
where <T as std::str::FromStr>::Err: std::fmt::Debug {
	i.next()
		.unwrap()
		.unwrap()
		.split_whitespace()
		.map(|x| x.parse::<T>().unwrap())
		.collect::<Vec<T>>()
}

pub fn main() {
	let stdout = stdout();
	let mut writer = BufWriter::new(stdout.lock());
	macro_rules! print { ($($x:tt)*) => { write!(writer, $($x)*).unwrap() }; }
	macro_rules! println { ($($x:tt)*) => { writeln!(writer, $($x)*).unwrap() }; }

	let input = std::io::stdin();
	let mut stdin = input.lock().lines();

	let count: i64 = read(&mut stdin);
	for _ in 0..count {
		let (d, k) = {
			let a: Vec<u64> = read_vec(&mut stdin);
			(a[0], a[1])
		};
		if is_utkarsh_wins(d, k) {
			println!("Utkarsh");
		} else {
			println!("Ashish");
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(is_utkarsh_wins(2, 1), true);
		assert_eq!(is_utkarsh_wins(5, 2), false);
		assert_eq!(is_utkarsh_wins(10, 3), true);
		assert_eq!(is_utkarsh_wins(25, 4), true);
		assert_eq!(is_utkarsh_wins(15441, 33), false);
	}
}
