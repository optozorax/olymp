#[fastio::fastio]
pub fn main() {
	let t = read!(usize);
	for _ in 0..t {
		let n = read!(usize);
		let arr = readln!(usize);
		let mut counter: Vec<Option<u64>> = vec![None; n];
		for i in 0..n {
			let current = arr[i];
			let has_left_to_delete = i > 0 && arr[i - 1] != current;
			let has_right_to_delete = arr.get(i + 1).map(|x| *x != current).unwrap_or(false);
			if let Some(to_change) = &mut counter[current - 1] {
				*to_change += has_right_to_delete as u64;
			} else {
				counter[current - 1] = Some(has_right_to_delete as u64 + has_left_to_delete as u64);
			}
		}
		println!("{}", counter.into_iter().filter_map(|x| x).min().unwrap());
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
