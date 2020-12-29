use rand::prelude::*;

#[fastio::fastio]
pub fn main() {
	let mut rng = rand::thread_rng();
	// -------------------------------------------------------------------- //
	//                      DO NOT FORGET TO FLUSH!!!                       //
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

include!("%/imports.rs");
include!("%/scanner.rs");
