use std::iter::repeat;

#[fastio::fastio]
pub fn main() {
	let t = read!(usize);
	for _ in 0..t {
		let n = read!(usize);

		#[derive(Default, Clone, Debug)]
		struct AllPlays {
			tie: usize,
			win: usize,
			lose: usize,
		}

		let each = AllPlays { tie: ((n - 1) % 2 == 1).into(), win: (n - 1) / 2, lose: (n - 1) / 2 };

		#[derive(Default, Clone, Debug)]
		struct Team {
			plays: AllPlays,
			array: BTreeMap<usize, i8>,
		}

		impl Team {
			fn add_check(&mut self, team: usize, result: i8, each: &AllPlays) -> bool {
				let not_tie = result == 0 && self.plays.tie == each.tie;
				let not_win = result == 1 && self.plays.win == each.win;
				let not_lose = result == -1 && self.plays.lose == each.lose;
				if not_tie || not_win || not_lose {
					return false;
				} else {
					self.add(team, result);
					return true;
				}
			}

			fn add(&mut self, team: usize, result: i8) {
				self.array.insert(team, result);
				if result == 0 {
					self.plays.tie += 1;
				} else if result == 1 {
					self.plays.win += 1;
				} else {
					self.plays.lose += 1;
				}
			}
		}

		let mut results: BTreeMap<usize, Team> = Default::default();

		for i in 0..n {
			let current = results.entry(i).or_insert_with(Default::default).clone();

			let should_add = repeat(0i8)
				.take(each.tie - current.plays.tie)
				.chain(repeat(1i8).take(each.win - current.plays.win))
				.chain(repeat(-1i8).take(each.lose - current.plays.lose))
				.collect::<Vec<i8>>();

			let mut remain_teams = (0..n)
				.filter(|x| *x != i)
				.filter(|x| !current.array.contains_key(x))
				.collect::<Vec<usize>>();

			'out: for result in should_add {
				for (pos, team) in remain_teams.iter().enumerate() {
					if results
						.entry(*team)
						.or_insert_with(Default::default)
						.add_check(i, -result, &each)
					{
						results.entry(i).or_insert_with(Default::default).add(*team, result);
						remain_teams.remove(pos);
						continue 'out;
					}
				}
				panic!("ice, {}, {}, {:?}", i, result, remain_teams);
			}
		}

		for i in 0..n {
			for j in i + 1..n {
				print!("{} ", results[&i].array[&j]);
			}
		}
		println!();
	}

	return;
	// solve by brute-force :D
	let n = 6;
	let games_count = n * (n - 1) / 2;
	let mut best_tie = games_count;
	let mut best_array = vec![];
	for_each_number(3, games_count, |results| {
		let mut scores = vec![0; n];
		let mut pos = 0;
		for i in 0..n {
			for j in i + 1..n {
				let score = results[pos];
				if score == 0 {
					scores[i] += 1;
					scores[j] += 1;
				} else if score == 1 {
					scores[i] += 3;
					scores[j] += 0;
				} else {
					scores[i] += 0;
					scores[j] += 3;
				}

				pos += 1;
			}
		}
		let tie_count = results.iter().filter(|x| **x == 0).count();
		let first = scores[0];
		let all_same_score = scores.iter().all(|x| *x == first);
		if all_same_score && tie_count < best_tie {
			best_tie = tie_count;
			best_array = results.to_vec();
		}
	});
	dbg!(best_tie, best_array);
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
include!("../../../../templates/src/to_include/for_each_number/base.rs");
