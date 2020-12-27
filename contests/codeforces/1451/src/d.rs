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

pub fn main() {
	// ----------------------------- Fast IO ------------------------------ //
	let stdout = stdout();
	let mut writer = BufWriter::new(stdout.lock());
	macro_rules! println { ($($x:tt)*) => { writeln!(writer, $($x)*).unwrap() }; }

	let input = stdin();
	let mut scanner = Scanner::new(input.lock().bytes().map(|x| x.unwrap()));
	#[rustfmt::skip] macro_rules! read { ($t:tt) => { scanner.read::<$t>() }; }
	// -------------------------------------------------------------------- //

	let count: i64 = read!(i64);
	for _ in 0..count {
		let d = read!(u64);
		let k = read!(u64);
		if is_utkarsh_wins(d, k) {
			println!("Utkarsh");
		} else {
			println!("Ashish");
		}
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
