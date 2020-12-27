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

pub fn main() {
	// ----------------------------- Fast IO ------------------------------ //
	let stdout = stdout();
	let mut writer = BufWriter::new(stdout.lock());
	macro_rules! println { ($($x:tt)*) => { writeln!(writer, $($x)*).unwrap() }; }

	let input = stdin();
	let mut scanner = Scanner::new(input.lock().bytes().map(|x| x.unwrap()));
	#[rustfmt::skip] macro_rules! read { ($t:tt) => { scanner.read::<$t>() }; }
	// -------------------------------------------------------------------- //

	let count = read!(i64);
	for _ in 0..count {
		let n = read!(i64);
		println!("{}", get_answer(n, 0));
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
