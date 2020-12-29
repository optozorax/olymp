#[fastio::fastio]
pub fn main() {
	let t = read!(usize);
	let mut inputs = Vec::new();
	for _ in 0..t {
		inputs.push(read!(usize));
	}

	println!("{}\n{}", t, Lines(inputs.clone()));
	flush!();

	for n in inputs {
		let readed = readln!(usize);
		if readed.len() == n && readed.iter().enumerate().all(|(index, v)| index + 1 != *v) {
			eprintln!("OK");
		} else {
			eprintln!("ERR");
		}
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
include!("../../../../templates/src/to_include/display/lines.rs");
