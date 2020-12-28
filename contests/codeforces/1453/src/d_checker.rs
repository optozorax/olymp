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
	let mut vec = Vec::new();
	for _ in 0..t {
		let is = read!(String) == "YES";
		let v = read!(u64);
		vec.push((is, v));
	}

	println!("{}", vec.len());
	for (_, i) in &vec {
		println!("{}", i);
	}
	flush!();

	for (is, value) in vec {
		let size = read!(i64);
		if !is {
			if size != -1 {
				eprintln!("ERR this is impossible");
				return;
			} else {
				// ok
			}
		} else {
			if size == -1 {
				eprintln!("ERR this possible");
				return;
			} else {
				let vec = readln!(u8);
				if vec[0] != 1 {
					eprintln!("ERR first elem must be 1");
					return;
				}
				let mut result = 0;
				let mut zeros_count = 0;
				for i in vec.iter().copied().skip(1) {
					if i == 1 {
						result += 2u64.pow(zeros_count + 2) - 2;
						zeros_count = 0;
					} else if i == 0 {
						zeros_count += 1;
					} else {
						eprintln!("ERR elem must be 0 or 1");
						return;
					}
				}
				result += 2u64.pow(zeros_count + 2) - 2;
				if result != value {
					eprintln!("ERR wrong number, expected: {}, got: {}", value, result);
					return;
				}
			}
		}
	}
	eprintln!("OK");
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
