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

	let t = read!(usize);
	for _ in 0..t {
		let _n = read!(usize);
		let mut r = readln!(i32);
		let _m = read!(usize);
		let mut b = readln!(i32);
		r.push(0);
		b.push(0);

		let max1 = r
			.iter()
			.scan(0, |acc, x| {
				let ans = *acc;
				*acc += x;
				Some(ans)
			})
			.max()
			.unwrap();

		let max2 = b
			.iter()
			.scan(0, |acc, x| {
				let ans = *acc;
				*acc += x;
				Some(ans)
			})
			.max()
			.unwrap();

		println!("{}", max1 + max2);
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
