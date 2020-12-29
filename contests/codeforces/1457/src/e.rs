#[fastio::fastio]
pub fn main() {
	let _n = read!(usize);
	let k = read!(usize);
	let mut c = readln!(i64);
	c.sort_by_key(|x| Reverse(*x));

	let mut heap = BinaryHeap::new();
	(0..=k).for_each(|_| heap.push(0));

	let mut count = 0;
	for i in c {
		let bonus = heap.pop().unwrap();
		count += bonus;
		heap.push(i + bonus);
	}

	println!("{}", count);
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
