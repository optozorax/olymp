#[fastio::fastio]
pub fn main() {
	let primes = {
		let mut res = Vec::new();
		for i in 2..100 {
			let prime = (|| {
				for j in &res {
					if i % j == 0 {
						return false;
					}
				}
				true
			})();
			if prime {
				res.push(i);
			}
		}
		res
	};

	let n = read!(u64);
	let ans = (|| {
		for i in &primes {
			for j in &primes {
				if i + j == n {
					return Some((*i, *j));
				}
			}
		}
		None
	})();
	if let Some((a, b)) = ans {
		println!("{} {}", a, b);
	} else {
		println!("-1");
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
