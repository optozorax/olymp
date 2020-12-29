use rand::prelude::*;

use crate::c2::*;

fn distance(a: &[Vec<Elem>], b: &[Vec<Elem>]) -> usize {
	a.iter()
		.zip(b.iter())
		.map(|(a, b)| {
			a.iter()
				.zip(b.iter())
				.map(|(a, b)| {
					if a != b {
						if *a != Elem::None && *b != Elem::None {
							1
						} else {
							eprintln!("ERR can't change . to X or to O");
							panic!();
						}
					} else {
						0
					}
				})
				.sum::<usize>()
		})
		.sum()
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
	//                      DO NOT FORGET TO FLUSH!!!                       //
	// -------------------------------------------------------------------- //
	//                      DO NOT FORGET TO FLUSH!!!                       //
	// -------------------------------------------------------------------- //

	let t = read!(usize);
	let mut fields = Vec::new();
	for _ in 0..t {
		let n = read!(usize);
		let mut field = Vec::with_capacity(n);
		for _ in 0..n {
			let a = bytes!().into_iter().map(Elem::from).collect::<Vec<_>>();
			field.push(a);
		}
		fields.push(field);
	}

	println!("{}", fields.len());
	for field in &fields {
		println!("{}", field.len());
		println!("{}", field.iter().map(|x| x.iter().joined_by("")).joined_by('\n'))
	}
	flush!();

	for input in fields {
		let n = input.len();
		let mut field = Vec::with_capacity(n);
		for _ in 0..n {
			let a = bytes!().into_iter().map(Elem::from).collect::<Vec<_>>();
			if a.len() != n {
				eprintln!("ERR wrong line size");
				return;
			}
			field.push(a);
		}
		if !check(&field) {
			eprintln!("ERR not correct");
			return;
		}
		let xo_count = count_xo(&input);
		let distance = distance(&input, &field);
		if distance > xo_count / 3 {
			eprintln!("ERR too many operations: {} > {} / 3", distance, xo_count);
			return;
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
