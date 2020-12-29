fn solve_fast(a: &[(u32, u32)]) -> usize {
	let starts = {
		let mut starts = a.iter().map(|(start, _)| start).collect::<Vec<_>>();
		starts.sort_unstable();
		starts
	};
	let ends = {
		let mut ends = a.iter().map(|(_, end)| end).collect::<Vec<_>>();
		ends.sort_unstable();
		ends
	};

	let mut min = a.len() - 1;
	for (start, end) in a {
		let before_this = binary_search(0..ends.len(), |pos| ends[pos] >= start).unwrap_or(0);
		let after_this = starts.len() - binary_search(0..starts.len(), |pos| starts[pos] > end).unwrap_or(starts.len());
		let count = a.len() - (before_this + after_this);
		min = std::cmp::min(min, a.len() - count);
	}

	min
}

#[fastio::fastio]
pub fn main() {
	let t = read!(usize);
	for _ in 0..t {
		let n = read!(usize);
		let a = {
			let mut a: Vec<(u32, u32)> = Vec::with_capacity(n);
			for _ in 0..n {
				let x = read!(u32);
				let y = read!(u32);
				a.push((x, y));
			}
			a
		};

		println!("{}", solve_fast(&a));
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
include!("../../../../templates/src/to_include/binary_search/base.rs");
