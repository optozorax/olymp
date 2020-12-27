fn solve(x: i64) -> i64 {
	let a = ArithmeticProgresion::new(1, 1);
	let pos = a.from_sum(x);
	let sum = a.sum_to(pos);

	let sum1 = a.sum_to(pos + 1);
	if x == sum {
		pos
	} else if x == sum1 - 1 {
		pos + 2
	} else {
		pos + 1
	}
}

pub fn main() {
	// ----------------------------- Fast IO ------------------------------ //
	let stdout = stdout();
	let mut writer = BufWriter::new(stdout.lock());
	macro_rules! println { ($($x:tt)*) => { writeln!(writer, $($x)*).unwrap() }; }

	let input = stdin();
	let mut scanner = Scanner::new(input.lock().bytes().map(|x| x.unwrap()));
	#[rustfmt::skip] macro_rules! read { ($t:tt) => { scanner.read::<$t>() }; }
	// -------------------------------------------------------------------- //

	let t = read!(usize);
	for _ in 0..t {
		let x = read!(i64);
		println!("{}", solve(x));
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/ \/ TESTING CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
//----------------------------------------------------------------------------

fn solve_brute_force(x: i64, k: i64, y: i64) -> usize {
	if y == x {
		return 0;
	}
	if k > 20 {
		return 100;
	}
	1 + min(solve_brute_force(x, k + 1, y + k), solve_brute_force(x, k + 1, y - 1))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn name() {
		for i in 0..40 {
			eprintln!("{}: {}, {}", i, solve_brute_force(i, 1, 0), solve(i));
			assert_eq!(solve_brute_force(i, 1, 0) as i64, solve(i));
		}
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
include!("../../../../templates/src/to_include/quadratic_equation.rs");
include!("../../../../templates/src/to_include/arithmetic_progression.rs");
