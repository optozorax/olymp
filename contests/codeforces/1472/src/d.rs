#[fastio::fastio]
pub fn main() {
	let t = read!(usize);
	for _ in 0..t {
		let _n = read!(usize);
		let mut a = readln!(i64); // SOOOOOOOQUA, у меня во время контеста не приняло, потому что я тут написал usize. И на дурацких компах кф эти числа оверфловнулись. ААААААААААААААААААААААААААААААААА. Больно.
		a.sort_unstable_by_key(|x| Reverse(*x));
		let mut alice = 0;
		let mut bob = 0;
		for (who, elem) in a.iter().enumerate().map(|(index, v)| (index % 2 == 0, v)) {
			if who {
				if elem % 2 == 0 {
					alice += elem;
				}
			} else {
				if elem % 2 == 1 {
					bob += elem;
				}
			}
		}
		match alice.cmp(&bob) {
			Ordering::Less => println!("Bob"),
			Ordering::Equal => println!("Tie"),
			Ordering::Greater => println!("Alice"),
		}
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
