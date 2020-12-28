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
		let n = read!(usize);
		if n == 3 {
			println!("2\n3 2\n3 2");
		} else if n == 4 {
			println!("3\n3 4\n4 2\n4 2");
		} else {
			// n >= 5
			let mut ops = Vec::new();
			ops.push((3, n));
			for i in 5..=min(15, n - 1) {
				ops.push((i, n));
			}
			for i in min(17, n)..n {
				ops.push((i, n));
			}
			let divisor = if n > 16 { 16 } else { 4 };
			let mut last = n;
			while last != 1 {
				ops.push((n, divisor));
				last = ceil_div(last, divisor);
			}
			if divisor == 16 {
				ops.push((16, 4));
				ops.push((16, 4));
			}
			ops.push((4, 2));
			ops.push((4, 2));

			println!("{}", ops.len());
			for (x, y) in ops {
				println!("{} {}", x, y);
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
include!("../../../../templates/src/to_include/math/ceil_div.rs");
