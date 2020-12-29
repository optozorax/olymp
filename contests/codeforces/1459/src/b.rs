#[fastio::fastio]
pub fn main() {
	let n = read!(i64);
	let answer = 1 + (3 * n * (n + 4) + 2 - (-1i64).pow(n as u32) * (n * (n + 4) + 2)) / 8;
	println!("{}", answer);

	return;

	#[allow(unreachable_code)]
	// Find answer using OEIS
	for n in 1..15i64 {
		let mut points = BTreeSet::new();
		for current_dir in [(1, 0), (-1, 0), (0, 1), (0, -1)].iter() {
			for_each_number(2, n as usize, |number| {
				let mut pos = (0, 0);
				let mut dir = *current_dir;
				for to_left in number {
					pos.0 += dir.0;
					pos.1 += dir.1;
					if *to_left == 0 {
						dir = (-dir.1, -dir.0);
					} else {
						dir = (dir.1, dir.0);
					}
				}
				points.insert(pos);
			});
		}
		let answer = 1 + (3 * n * (n + 4) + 2 - (-1i64).pow(n as u32) * (n * (n + 4) + 2)) / 8;
		assert_eq!(answer as usize, points.len());
		println!("{}: {}, {}", n, points.len(), answer);
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
include!("../../../../templates/src/to_include/for_each_number/base.rs");
