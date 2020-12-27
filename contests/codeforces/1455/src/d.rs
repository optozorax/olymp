fn solve(mut x: i64, mut a: Vec<i64>) -> Option<usize> {
	let mut count = 0;
	loop {
		let mut copy = a.clone();
		copy.sort_unstable();
		if copy == a {
			return Some(count);
		}
		count += 1;
		let pos = a
			.iter()
			.copied()
			.enumerate()
			.filter(|(_, i)| *i > x)
			.find(|(index, _)| {
				index
					.checked_sub(1)
					.and_then(|i| a.get(i))
					.map(|i| *i <= x)
					.unwrap_or(true) || a.get(index + 1).map(|i| x <= *i).unwrap_or(true)
			})?
			.0;
		std::mem::swap(&mut a[pos], &mut x);
	}
}

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
		let _n = read!(i64);
		let x = read!(i64);
		let a = readln!(i64);
		if let Some(count) = solve(x, a) {
			println!("{}", count);
		} else {
			println!("-1");
		}
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
