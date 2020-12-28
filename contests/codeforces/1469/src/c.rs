fn solve(h: &[i64], k: i64) -> bool {
	let mut current_range = (h[0], h[0]);
	for h in h.iter().copied().skip(1).take(h.len() - 2) {
		let h_range = (h, h + (k - 1));
		current_range = (max(current_range.0 - (k - 1), h_range.0), min(current_range.1 + (k - 1), h_range.1));
		if current_range.0 > current_range.1 {
			return false;
		}
	}
	let last = *h.last().unwrap();
	current_range.0 - (k - 1) <= last && last <= current_range.1 + (k - 1)
}

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
		let _n = read!(i64);
		let k = read!(i64);
		let h = readln!(i64);
		if solve(&h, k) {
			println!("YES");
		} else {
			println!("NO");
		}
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
