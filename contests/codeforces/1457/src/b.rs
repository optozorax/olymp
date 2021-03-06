#[fastio::fastio]
pub fn main() {
	let t = read!(usize);
	for _ in 0..t {
		let _n = read!(u64);
		let k = read!(u64);
		let a = readln!(usize);

		#[derive(Clone, Debug)]
		struct C {
			count: u64,
			steps: u64,
		}
		let mut c = vec![C { count: 0, steps: 0 }; 101];
		let current_colors = a
			.iter()
			.copied()
			.collect::<BTreeSet<_>>()
			.into_iter()
			.collect::<Vec<_>>();

		for current in a {
			for &pos in &current_colors {
				if c[pos].steps == 0 {
					if pos != current {
						c[pos].steps = k;
						c[pos].count += 1;
					}
				} else {
					c[pos].steps -= 1;
					if c[pos].steps == 0 && pos != current {
						c[pos].steps = k;
						c[pos].count += 1;
					}
				}
			}
		}

		let max = current_colors.iter().map(|&pos| c[pos].count).min().unwrap();
		println!("{}", max);
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
