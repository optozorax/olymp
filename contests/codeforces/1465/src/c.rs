#[fastio::fastio]
pub fn main() {
	#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
	struct Rook {
		x: usize,
		y: usize,
	}

	#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
	enum DiagState {
		Empty,
		Taken,
		BeatenOne(Rook),
		BeatenTwo(Rook, Rook),
	}

	use DiagState::*;

	let t = read!(usize);
	for _ in 0..t {
		let n = read!(usize);
		let m = read!(usize);
		let mut d = vec![Empty; n];
		for _ in 0..m {
			let mut x = read!(usize);
			let mut y = read!(usize);
			x -= 1;
			y -= 1;
			if x == y {
				d[x] = Taken;
			} else {
				d[x] = match d[x] {
					Empty => BeatenOne(Rook { x, y }),
					BeatenOne(a) => BeatenTwo(a, Rook { x, y }),
					_ => unreachable!(),
				};
				d[y] = match d[y] {
					Empty => BeatenOne(Rook { x, y }),
					BeatenOne(a) => BeatenTwo(a, Rook { x, y }),
					_ => unreachable!(),
				};
			}
		}

		let mut empty_count = d.iter().filter(|x| matches!(x, Empty)).count();
		let mut to_recalc_ones = d
			.iter()
			.filter_map(|x| match x {
				BeatenOne(r) => Some(*r),
				_ => None,
			})
			.collect::<BTreeSet<Rook>>();
		let mut to_recalc_twos = d
			.iter()
			.filter_map(|x| match x {
				BeatenTwo(a, b) => Some((*a, *b)),
				_ => None,
			})
			.collect::<BTreeSet<(Rook, Rook)>>();

		let mut steps = 0;
		while !to_recalc_ones.is_empty() || !to_recalc_twos.is_empty() {
			if !to_recalc_ones.is_empty() {
				let current = *to_recalc_ones.iter().next().unwrap();
				to_recalc_ones.remove(&current);

				let (take_pos, leave_pos) = if matches!(d[current.x], BeatenOne(_)) {
					(current.x, current.y)
				} else {
					assert!(matches!(d[current.y], BeatenOne(_)));
					(current.y, current.x)
				};

				d[take_pos] = Taken;
				d[leave_pos] = match d[leave_pos] {
					Empty => unreachable!(),
					Taken => unreachable!(),
					BeatenOne(_) => {
						steps += 1;
						empty_count += 1;
						Empty
					},
					BeatenTwo(a, b) => {
						steps += 1;
						to_recalc_twos.remove(&(a, b));
						let remains = if a == current { b } else { a };
						to_recalc_ones.insert(remains);
						BeatenOne(remains)
					},
				};
			} else {
				let current = *to_recalc_twos.iter().next().unwrap();

				assert!(empty_count != 0);
				empty_count -= 1;
				steps += 2;

				let pos1 = current.0.x;
				let pos2 = current.0.y;

				assert!(pos1 != pos2);

				d[pos1] = match d[pos1] {
					BeatenTwo(a, b) => {
						to_recalc_twos.remove(&(a, b));
						let remains = if a == current.0 { b } else { a };
						to_recalc_ones.insert(remains);
						BeatenOne(remains)
					},
					_ => unreachable!(),
				};
				d[pos2] = match d[pos2] {
					BeatenTwo(a, b) => {
						to_recalc_twos.remove(&(a, b));
						let remains = if a == current.0 { b } else { a };
						to_recalc_ones.insert(remains);
						BeatenOne(remains)
					},
					_ => unreachable!(),
				};
			}
		}
		println!("{}", steps);
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
