#[fastio::fastio]
pub fn main() {
	let t = read!(usize);
	for _ in 0..t {
		let n = read!(usize);
		let mut a = Vec::with_capacity(n);
		let mut ws = Vec::with_capacity(n);
		let mut hs = Vec::with_capacity(n);
		for i in 0..n {
			let w = read!(u64);
			let h = read!(u64);
			ws.push((w, h, i));
			hs.push((h, w, i));
			a.push((w, h));
		}
		ws.sort_by_key(|x| Reverse(x.0));
		hs.sort_by_key(|x| Reverse(x.0));

		let ws_suf = Suffix::new(&ws, |a, b| if a.1 < b.1 { a } else { b });
		let hs_suf = Suffix::new(&hs, |a, b| if a.1 < b.1 { a } else { b });

		for (index, (w, h)) in a.into_iter().enumerate() {
			let ans = binary_search(0..ws.len(), |pos| ws[pos].0 < w)
				.and_then(|pos| ws_suf.get_suffix_f(pos))
				.filter(|x| x.1 < h)
				.map(|x| x.2)
				.filter(|x| *x != index)
				.or_else(|| {
					binary_search(0..ws.len(), |pos| ws[pos].0 < h)
						.and_then(|pos| ws_suf.get_suffix_f(pos))
						.filter(|x| x.1 < w)
						.map(|x| x.2)
						.filter(|x| *x != index)
				})
				.or_else(|| {
					binary_search(0..hs.len(), |pos| hs[pos].0 < w)
						.and_then(|pos| hs_suf.get_suffix_f(pos))
						.filter(|x| x.1 < h)
						.map(|x| x.2)
						.filter(|x| *x != index)
				})
				.or_else(|| {
					binary_search(0..hs.len(), |pos| hs[pos].0 < h)
						.and_then(|pos| hs_suf.get_suffix_f(pos))
						.filter(|x| x.1 < w)
						.map(|x| x.2)
						.filter(|x| *x != index)
				});
			if let Some(pos) = ans {
				print!("{} ", pos + 1);
			} else {
				print!("-1 ");
			}
		}
		println!();
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
include!("../../../../templates/src/to_include/suffix.rs");
include!("../../../../templates/src/to_include/binary_search/base.rs");
