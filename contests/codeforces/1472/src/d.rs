fn solve(mut a: Vec<i64>) -> Ordering {
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
	alice.cmp(&bob)
}

#[fastio::fastio]
pub fn main() {
	let t = read!(usize);
	for _ in 0..t {
		let _n = read!(usize);
		let a = readln!(i64);
		let ans = solve(a);
		match ans {
			Ordering::Less => println!("Bob"),
			Ordering::Equal => println!("Tie"),
			Ordering::Greater => println!("Alice"),
		}
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/ \/ TESTING CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
//----------------------------------------------------------------------------

fn solve_slow(a: Vec<i64>, print: bool) -> Ordering {
	#[derive(Clone, Debug)]
	struct ThisGame {
		steps: Vec<i64>,
		inner: Vec<i64>,
		alice: i64,
		bob: i64,
		i: usize,
	}

	impl OptimalGame for ThisGame {
		type CurrentStepsIter = Range<usize>;
		type StepInfo = usize;

		fn iter_current_steps(&self) -> Self::CurrentStepsIter { 0..self.inner.len() }

		fn step(&mut self, info: Self::StepInfo) {
			let elem = self.inner[info];
			self.steps.push(elem);
			self.inner.remove(info);
			if self.i % 2 == 0 {
				if elem % 2 == 0 {
					self.alice += elem;
				}
			} else {
				if elem % 2 == 1 {
					self.bob += elem;
				}
			}
			self.i += 1;
		}

		fn who_wins(&self) -> GameResult {
			if self.is_done() {
				GameResult::Done(match self.alice.cmp(&self.bob) {
					Ordering::Less => DoneResult::OneWinner(Player::Second),
					Ordering::Equal => DoneResult::Tie,
					Ordering::Greater => DoneResult::OneWinner(Player::First),
				})
			} else {
				GameResult::NotDone
			}
		}

		fn is_done(&self) -> bool { self.inner.is_empty() }

		fn current_player(&self) -> Player { if self.i % 2 == 0 { Player::First } else { Player::Second } }
	}

	let game = ThisGame { steps: Vec::new(), inner: a, alice: 0, bob: 0, i: 0 };
	let result = play_optimally_slow(game);

	if print {
		for game in result.iter() {
			eprintln!("{}", SpaceVec(game.steps.clone()));
		}
	}

	result[0].alice.cmp(&result[0].bob)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn name() {
		solve_slow(vec![1, 2, 3], true);
		eprintln!();
		solve_slow(vec![51, 8, 7], true);
		eprintln!();
		solve_slow(vec![51, 8, 7, 5, 3, 6, 68], true);
		eprintln!();
		solve_slow(vec![5, 7, 2, 3], true);
		eprintln!();
		solve_slow(vec![1, 1, 2], true);
		eprintln!();
		solve_slow(vec![2, 7, 2], true);
		panic!();
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
include!("../../../../templates/src/to_include/display/space_vec.rs");
include!("../../../../templates/src/to_include/optimal_game.rs");
