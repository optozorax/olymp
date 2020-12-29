fn solve(a: i64, b: i64, c: i64) -> bool {
	let super_count = (a + b + c) / 9;
	a >= super_count && b >= super_count && c >= super_count && (a + b + c) % 9 == 0
}

#[fastio::fastio]
pub fn main() {
	let t = read!(usize);
	for _ in 0..t {
		let a = read!(i64);
		let b = read!(i64);
		let c = read!(i64);
		if solve(a, b, c) {
			println!("YES");
		} else {
			println!("NO");
		}
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/ \/ TESTING CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
//----------------------------------------------------------------------------

macro_rules! check {
	($a:ident, $b:ident, $c:ident) => {
		if $a <= 0 && ($b != 0 || $c != 0) {
			return false;
		} else if $b <= 0 && ($a != 0 || $c != 0) {
			return false;
		} else if $c <= 0 && ($b != 0 || $a != 0) {
			return false;
		} else if $a == 0 && $b == 0 && $c == 0 {
			return true;
		}
	};
}

fn solve_slow(mut a: i64, mut b: i64, mut c: i64) -> bool {
	let mut i = 0;
	loop {
		i += 1;

		if i % 7 == 0 {
			a -= 1;
			b -= 1;
			c -= 1;
		} else if a >= b && a >= c {
			a -= 1;
		} else if b >= a && b >= c {
			b -= 1;
		} else {
			c -= 1;
		}

		check!(a, b, c);
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn name() {
		use std::time::{Duration, Instant};

		use rand::Rng;
		let mut rng = rand::thread_rng();
		for _ in 0..2000000 {
			let a = rng.gen_range(1, 100);
			let b = rng.gen_range(1, 100);
			let c = rng.gen_range(1, 100);
			let ans_slow = solve_slow(a, b, c);
			let ans_fast = solve(a, b, c);
			if ans_slow != ans_fast {
				dbg!(a, b, c, ans_slow, ans_fast);
				panic!();
			}
		}
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
