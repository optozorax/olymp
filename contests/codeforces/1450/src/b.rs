fn solve(points: &[(i64, i64)], k: i64) -> i64 {
	for i in 0..points.len() {
		let mut can_be = true;
		for j in 0..points.len() {
			if (points[i].0 - points[j].0).abs() + (points[i].1 - points[j].1).abs() > k {
				can_be = false;
				break;
			}
		}
		if can_be {
			return 1;
		}
	}
	-1
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
		let n = read!(i64);
		let k = read!(i64);
		let mut points = Vec::with_capacity(n as usize);
		for _ in 0..n {
			let x = read!(i64);
			let y = read!(i64);
			points.push((x, y));
		}
		println!("{}", solve(&points, k));
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
