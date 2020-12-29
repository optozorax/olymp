#[fastio::fastio]
pub fn main() {
	let count = read!(usize);

	enum Get {
		Xor,
		And,
	}

	let mut get = |kind: Get, i: usize, j: usize| -> usize {
		let s = match kind {
			Get::Xor => "XOR",
			Get::And => "AND",
		};
		println!("{} {} {}", s, i + 1, j + 1);
		flush!();
		read!(usize)
	};

	let xored = {
		let mut result: Vec<usize> = Vec::new();
		for i in 1..count {
			result.push(get(Get::Xor, 0, i))
		}
		result
	};

	let first = if let Some(zero_pos) = xored.iter().position(|x| *x == 0) {
		// there is same value as first
		xored[zero_pos] ^ get(Get::And, 0, zero_pos + 1)
	} else if let Some((a, b)) = xored
		.iter()
		.enumerate()
		.duplicates_with_vec_by(count, |(_, value)| *value)
		.map(|(a, b)| (a.0, b.0))
		.next()
	{
		// there is two same values
		xored[a] ^ get(Get::And, a + 1, b + 1)
	} else {
		// there is all different numbers
		let a = get(Get::And, 0, xored.iter().position(|x| *x == 1).unwrap() + 1);
		let b = get(Get::And, 0, xored.iter().position(|x| *x == 2).unwrap() + 1);
		a | b
	};

	print!("! {}", first);
	for i in xored {
		print!(" {}", i ^ first);
	}
	println!();
	flush!();
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
include!("../../../../templates/src/to_include/iterator/duplicate.rs");
