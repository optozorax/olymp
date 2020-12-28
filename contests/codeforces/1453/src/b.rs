pub fn main() {
	// ----------------------------- Fast IO ------------------------------ //
	let stdout = stdout();
	let mut writer = BufWriter::new(stdout.lock());
	macro_rules! println { ($($x:tt)*) => { writeln!(writer, $($x)*).unwrap() }; }

	let input = stdin();
	let mut scanner = Scanner::new(input.lock().bytes().map(|x| x.unwrap()));
	#[rustfmt::skip] macro_rules! read { ($t:tt) => { scanner.read::<$t>() }; }
	#[rustfmt::skip] macro_rules! readln { ($t:tt) => { scanner.readln::<$t>() }; }
	// -------------------------------------------------------------------- //

	let t = read!(i64);
	for _ in 0..t {
		let _n = read!(i64);
		let a = readln!(i64);
		let mut max = None;
		if a.len() >= 2 {
			max = max.any_or_both_with(Some((a[0] - a[1]).abs()), std::cmp::max);
			max = max.any_or_both_with(Some((a[a.len() - 1] - a[a.len() - 2]).abs()), std::cmp::max);
		}
		max = max.any_or_both_with(
			a.windows(3)
				.map(|w| {
					let a = w[0];
					let b = w[1];
					let c = w[2];
					let optimal_b = (a + c) / 2;
					(((a - b).abs() + (b - c).abs()) - ((a - optimal_b).abs() + (optimal_b - c).abs())).abs()
				})
				.max(),
			std::cmp::max,
		);
		let max = max.unwrap_or(0);
		let actions: i64 = a.windows(2).map(|w| (w[0] - w[1]).abs()).sum();
		println!("{}", actions - max);
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
include!("../../../../templates/src/to_include/option.rs");
