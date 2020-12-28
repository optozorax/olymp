use rand::prelude::*;

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

	let from = read!(usize);
	let to = read!(usize);

	let mut vec = (from..=to).collect::<Vec<_>>();
	let mut rng = rand::thread_rng();
	vec.shuffle(&mut rng);

	vec = vec.into_iter().take(100).collect::<Vec<_>>();

	println!("{}", vec.len());
	for n in &vec {
		println!("{}", n);
	}
	flush!();

	for n in &vec {
		let count = read!(usize);
		if count > n + 5 {
			eprintln!("ERR operations exceeded, on iter {}", n);
			return;
		} else {
			let mut vec = (1..=*n).collect::<Vec<_>>();
			for _ in 0..count {
				let x = read!(usize) - 1;
				let y = read!(usize) - 1;
				if x == y {
					eprintln!("ERR x == y is not allowed on iteration {}", n);
					return;
				}
				vec[x] = vec[x] / vec[y] + if vec[x] % vec[y] == 0 { 0 } else { 1 };
			}
			let mut two_was = false;
			for x in vec {
				if x == 2 {
					if two_was {
						eprintln!("ERR 2 not one time on iter {}", n);
						return;
					} else {
						two_was = true;
					}
				} else if x == 1 {
					// ok
				} else {
					eprintln!("ERR not 1 or 2: {}, on iter {}", x, n);
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
