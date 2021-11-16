#[fastio::fastio]
pub fn main() {
	let n = read!(usize);
	let m = read!(usize);
	let mut al = vec![false; n];
	let mut tl = SegmentTree::create(
		&mut al,
		|_, a: Option<usize>, b: Option<usize>| a.any_or_both_with(b, |a, b| a.max(b)),
		|id, x| if x { Some(id) } else { None },
	);
	let mut ar = vec![false; n];
	let mut tr = SegmentTree::create(
		&mut ar,
		|_, a: Option<usize>, b: Option<usize>| a.any_or_both_with(b, |a, b| a.min(b)),
		|id, x| if x { Some(id) } else { None },
	);
	for _ in 0..m {
		let c = read!(String);
		let pos = read!(usize) - 1;
		if c == "add" {
			tl.set(pos, true);
			tr.set(pos, true);
		} else if c == "get" {
			let mut best = n + 1;
			if let Some(Some(a)) = tl.f_for_segment(0..pos) {
				best = best.min(pos - a);
			}
			if let Some(Some(a)) = tr.f_for_segment(pos + 1..n) {
				best = best.min(a - pos);
			}
			if best == n + 1 {
				println!("-1");
			} else {
				println!("{}", best);
			}
		}
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
include!("../../../../templates/src/to_include/option.rs");
include!("../../../../templates/src/to_include/segment_tree.rs");
