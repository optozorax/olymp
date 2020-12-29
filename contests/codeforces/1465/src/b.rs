fn is_fair(mut n: u64) -> bool {
	let n_copy = n;
	std::iter::from_fn(|| {
		if n != 0 {
			let number = n % 10;
			n /= 10;
			Some(number)
		} else {
			None
		}
	})
	.filter(|x| *x != 0)
	.all(|x| n_copy % x == 0)
}

fn nearest_fair(n: u64) -> u64 {
	for i in n.. {
		if is_fair(i) {
			return i;
		}
	}
	unreachable!()
}

#[fastio::fastio]
pub fn main() {
	let t = read!(usize);
	for _ in 0..t {
		let n = read!(u64);
		let answer = nearest_fair(n);
		println!("{}", answer);
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
