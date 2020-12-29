enum CutResult<'a> {
	Two(&'a [i64], &'a [i64], MyRange, MyRange),
	One(&'a [i64], MyRange),
}

fn cut(arr: &[i64], at: MyRange) -> CutResult {
	let mid = (arr[0] + arr.last().unwrap()) / 2;
	if let Some(pos) = binary_search(0..arr.len(), |pos| arr[pos] > mid) {
		let (a, b) = arr.split_at(pos);
		let pos = pos as i64;
		if a.is_empty() {
			if b.is_empty() {
				panic!("it can't be");
			} else {
				CutResult::One(b, (at.start + pos..at.end).into())
			}
		} else if b.is_empty() {
			CutResult::One(a, (at.start..at.start + pos).into())
		} else {
			let (ar, br) = at.split(at.start + pos).unwrap();
			CutResult::Two(a, b, ar, br)
		}
	} else {
		CutResult::One(arr, at)
	}
}

#[fastio::fastio]
pub fn main() {
	let t = read!(i64);
	for _ in 0..t {
		let _n = read!(usize);
		let q = read!(usize);
		let mut a = readln!(i64);
		a.sort_unstable();
		let prefix = Prefix::new(&a, |x, y| x + y);

		let mut possible_sums: BTreeSet<i64> = BTreeSet::new();
		possible_sums.insert(a.iter().sum::<i64>());
		let mut changing: Vec<(&[i64], MyRange)> = vec![(&a[..], (0..a.len()).into())];
		let mut new_twos = vec![];
		while !changing.is_empty() {
			new_twos.clear();
			for (arr, at) in &changing {
				use CutResult::*;
				match cut(arr, *at) {
					Two(a, b, aat, bat) => {
						possible_sums.insert(prefix.get_segment_f(aat.into(), |x, y| x - y).unwrap());
						possible_sums.insert(prefix.get_segment_f(bat.into(), |x, y| x - y).unwrap());
						new_twos.push((a, aat));
						new_twos.push((b, bat));
					},
					One(_, at1) => {
						possible_sums.insert(prefix.get_segment_f(at1.into(), |x, y| x - y).unwrap());
					},
				}
			}
			std::mem::swap(&mut changing, &mut new_twos);
		}

		for _ in 0..q {
			let s = read!(i64);
			if possible_sums.contains(&s) {
				println!("Yes");
			} else {
				println!("No");
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
include!("../../../../templates/src/to_include/binary_search/base.rs");
include!("../../../../templates/src/to_include/prefix.rs");
include!("../../../../templates/src/to_include/my_range.rs");
