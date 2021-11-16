#[fastio::fastio]
pub fn main() {
	let n = read!(usize);
	let mut fav: HashMap<Vec<u8>, u64> = Default::default();
	let mut other: HashSet<Vec<u8>> = Default::default();
	let mut best_count: u64 = 0;
	for _ in 0..n {
		let mut s = bytes!();
		if s.last() == Some(&b'+') {
			s.pop();
			for i in 0..s.len() {
				for j in i + 1..=s.len() {
					*fav.entry((&s[i..j]).to_vec()).or_insert(0) |= 1 << best_count;
				}
			}
			best_count += 1;
		} else {
			for i in 0..s.len() {
				for j in i + 1..=s.len() {
					other.insert((&s[i..j]).to_vec());
				}
			}
		}
		let should_eq = (1 << best_count) - 1;
		// dbg!(&fav.entry(vec![b'k']), &other.contains(&vec![b'k']), should_eq);
		// eprintln!();
		let ans = fav
			.iter()
			.filter(|(_, c)| **c == should_eq)
			.filter(|(v, _)| !other.contains(*v))
			.min_by_key(|(v, _)| v.len());
		if let Some((v, _)) = ans {
			println!("{}", std::str::from_utf8(v).unwrap());
		} else {
			println!("-1");
		}
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
