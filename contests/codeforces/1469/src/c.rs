include!("../../../../templates/src/to_include/macro/try_bool.rs");

fn solve(h: &[i64], mut k: i64) -> bool {
	k -= 1;
	let end = try_bool!(
		h.iter()
			.copied()
			.skip(1)
			.take(h.len() - 2)
			.map(|hi| MyRange::from(hi..=hi + k))
			.try_fold(MyRange::point(h[0]), |acc, x| MyRange::from(acc.start - k..acc.end + k)
				.intersect(x)?
				.not_empty())
	);
	MyRange::from(end.start - k..end.end + k).contains(*h.last().unwrap())
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
include!("../../../../templates/src/to_include/my_range.rs");
