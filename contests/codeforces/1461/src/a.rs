#[fastio::fastio]
pub fn main() {
	let t = read!(usize);
	for _ in 0..t {
		let n = read!(usize);
		let k = read!(usize);

		let mut result = Vec::new();
		if k == 1 {
			result.push(b'b');
		} else {
			result.push(b'b');
			result.resize(k - 1, b'a');
			result.push(b'b');
		}
		let mut i = 0i64;
		while result.len() < n {
			if i % 3 == 0 {
				result.push(b'c');
			} else if i % 3 == 1 {
				result.push(b'a');
			} else {
				result.push(b'b');
			}
			i += 1;
		}

		writeln!(writer, "{}", Chars(result)).unwrap();
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
include!("../../../../templates/src/to_include/display/chars.rs");
