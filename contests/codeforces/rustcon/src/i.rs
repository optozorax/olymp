#[fastio::fastio]
pub fn main() {
	let m = read!(usize);
	let n = read!(usize);
	let k = read!(usize);
	let mut input: Vec<Vec<(usize, usize)>> = vec![Default::default(); k];
	let mut inv_input: Vec<Vec<(usize, usize)>> = vec![vec![(0, 0); n]; m];
	for (kpos, ki) in input.iter_mut().enumerate() {
		let a = read!(usize);
		ki.resize(a, (0, 0));
		for (apos, ai) in ki.iter_mut().enumerate() {
			let mi = read!(usize) - 1;
			let ni = read!(usize) - 1;
			*ai = (mi, ni);
			inv_input[mi][ni] = (kpos, apos);
		}
	}

	// dbg!(&inv_input);

	let mut maxs = vec![0; m];
	let mut to_visit = HashSet::new();
	for (pos, i) in input.iter().enumerate() {
		if i.last().map(|x| x.1 == maxs[x.0]).unwrap_or(false) {
			to_visit.insert(pos);
		}
	}

	let mut to_visit2 = HashSet::new();
	let mut result: Vec<(usize, usize)> = Vec::new();
	let mut lasts = Vec::new();
	while !to_visit.is_empty() {
		to_visit2.clear();
		lasts.clear();

		// dbg!(&to_visit);

		for i in &to_visit {
			if let Some(last) = input[*i].last() {
				result.push(*last);
				maxs[last.0] = last.1 + 1;
				lasts.push(*last);
				input[*i].pop();
			}
		}

		for (i, last) in to_visit.iter().zip(lasts.iter()) {
			if let Some(x) = input[*i].last() {
				if x.1 == maxs[x.0] {
					to_visit2.insert(*i);
				}
			}
			if last.1 != n - 1 {
				let (kpos, apos) = inv_input[last.0][last.1 + 1];
				// dbg!(last, kpos, apos, input[kpos].len());
				if input[kpos].len() == apos + 1 {
					to_visit2.insert(kpos);
				}
			}
		}
		std::mem::swap(&mut to_visit, &mut to_visit2);
	}

	// dbg!(&maxs);

	if maxs.iter().all(|x| *x == n) {
		println!("YES");
		for i in result {
			println!("{} {}", i.0 + 1, i.1 + 1);
		}
	} else {
		println!("NO");
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
