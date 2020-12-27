#[allow(clippy::mut_range_bound)]
fn factorize(mut n: u64) -> BTreeMap<u64, usize> {
	let mut result = BTreeMap::new();
	for i in 2..n {
		if i > n || i > (n as f64).sqrt() as u64 + 5 {
			break;
		}
		while n % i == 0 {
			*result.entry(i).or_insert(0) += 1;
			n /= i;
		}
	}
	if n != 0 {
		*result.entry(n).or_insert(0) += 1;
	}
	result
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
		let mut n = read!(u64);
		let max_count = factorize(n)
			.into_iter()
			.max_by_key(|(_miltiprier, count)| *count)
			.unwrap();
		println!("{}", max_count.1);
		for _ in 0..max_count.1 - 1 {
			print!("{} ", max_count.0);
			n /= max_count.0;
		}
		println!("{}", n);
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
