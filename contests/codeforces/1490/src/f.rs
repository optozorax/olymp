#[fastio::fastio]
pub fn main() {
	let t = read!(usize);
	for _ in 0..t {
		let _n = read!(usize);
		let a = readln!(i64);
		let mut map: BTreeMap<i64, i64> = BTreeMap::new();
		for i in a {
			*map.entry(i).or_insert(0) += 1;
		}
		let mut a = map.into_iter().map(|(_, x)| x).collect::<Vec<_>>();
		a.sort_unstable();

		let f = |i: usize| {
			let need = a[i];
			let count: i64 = a.iter().map(|x| if *x < need { *x } else { *x - need }).sum();
			count
		};

		let mut answer = binary_search(0..a.len() - 1 as usize, |i| f(i) - f(i + 1) < 0).unwrap_or(a.len() - 1);

		if f(0) < f(answer) {
			answer = 0;
		}

		println!("{}", f(answer));
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
include!("../../../../templates/src/to_include/binary_search/base.rs");
