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

	let arr = readln!(usize);
	println!("{}", arr.len());
	flush!();

	let n = arr.len();

	for _ in 0..n + 2 {
		let op = read!(String);
		match &op[..] {
			"AND" => {
				let i = read!(usize) - 1;
				let j = read!(usize) - 1;
				println!("{}", arr[i] & arr[j]);
			},
			"OR" => {
				let i = read!(usize) - 1;
				let j = read!(usize) - 1;
				println!("{}", arr[i] | arr[j]);
			},
			"XOR" => {
				let i = read!(usize) - 1;
				let j = read!(usize) - 1;
				println!("{}", arr[i] ^ arr[j]);
			},
			"!" => {
				let result = readln!(usize);
				if result == arr {
					eprintln!("OK");
				} else {
					eprintln!("ERR: wrong answer");
				}
				return;
			},
			_ => {
				eprintln!("ERR: unknown operation `{}`", op);
				return;
			},
		}
		flush!();
	}
	eprintln!("ERR: max count of operations exceeded");
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
