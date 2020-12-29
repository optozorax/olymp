#[fastio::fastio]
pub fn main() {
	let size = read!(usize);
	println!("{}", size);
	for _ in 0..size {
		println!("0 0\n0 1000000000\n1000000000 0\n1000000000 1000000000");
	}
	flush!();

	for _ in 0..size {
		let ans = read!(usize);
		if ans != 0 {
			eprintln!("ERR");
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
