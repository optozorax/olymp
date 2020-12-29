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
		let n = read!(usize);
		let m = read!(usize);
		let matrix = {
			let mut matrix = Vec::new();
			for _ in 0..n {
				let line = bytes!().into_iter().map(|x| x == b'*').collect::<Vec<bool>>();
				matrix.push(line);
			}
			matrix
		};

		let pre_calculated = {
			let mut pre_calculated = vec![vec![0usize; m]; n];
			for x in 0..n {
				let mut count = 0;
				for y in 0..m {
					if matrix[x][y] {
						count += 1;
					} else {
						count = 0;
					}
					pre_calculated[x][y] = count;
				}
			}
			pre_calculated
		};

		let if_el = |x: usize, y: usize, k: usize| {
			pre_calculated
				.get(x + k)
				.and_then(|line| line.get(y + k))
				.map(|count| *count > k * 2)
				.unwrap_or(false)
		};

		let mut count = 0;
		for x in 0..n {
			for y in 0..m {
				for k in 0..n {
					if if_el(x, y, k) {
						count += 1;
					} else {
						break;
					}
				}
			}
		}

		println!("{}", count);
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
