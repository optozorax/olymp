pub fn main() {
	// ----------------------------- Fast IO ------------------------------ //
	let stdout = stdout();
	let mut writer = BufWriter::new(stdout.lock());
	macro_rules! print { ($($x:tt)*) => { write!(writer, $($x)*).unwrap() }; }
	macro_rules! println { ($($x:tt)*) => { writeln!(writer, $($x)*).unwrap() }; }
	#[rustfmt::skip] macro_rules! flush { ($($x:tt)*) => { writer.flush().unwrap() }; }

	let input = stdin();
	let mut scanner = Scanner::new(input.lock().bytes().map(|x| x.unwrap()));
	#[rustfmt::skip] macro_rules! read { ($t:tt) => { scanner.read::<$t>() }; }
	#[rustfmt::skip] macro_rules! readln { ($t:tt) => { scanner.readln::<$t>() }; }
	#[rustfmt::skip] macro_rules! byte { () => { scanner.byte() }; }
	#[rustfmt::skip] macro_rules! bytes { () => { scanner.bytes() }; }
	// -------------------------------------------------------------------- //

	let t = read!(usize);
	for _ in 0..t {
		let _n = read!(usize);
		let m = read!(usize);
		let a = readln!(usize);
		let not_sorted_pos = a.len()
			- a.iter()
				.copied()
				.enumerate()
				.map(|(index, val)| (index + 1, val))
				.rev()
				.position(|(index, val)| index != val)
				.unwrap_or(0);

		let is_sorted = a
			.iter()
			.copied()
			.enumerate()
			.map(|(index, val)| (index + 1, val))
			.all(|(index, val)| index == val);

		let mut result = if is_sorted { 1.0f64 } else { 0.0f64 };
		let mut current_probability = if is_sorted { 0.0f64 } else { 1.0f64 };
		for _ in 0..m {
			let r = read!(usize);
			let p = read!(f64);
			if r >= not_sorted_pos {
				result += p * current_probability;
				current_probability *= 1.0 - p;
			}
		}

		println!("{}", result);
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
