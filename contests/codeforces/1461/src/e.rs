#[derive(Clone, Debug)]
struct Input {
	current: i64,
	max: i64,
	need_days: i64,
	can_add: i64,
	must_sub: i64,
}

impl Input {
	// 0..=result can add can_add
	fn where_can_add(&self) -> Option<i64> {
		let result = self.max - self.can_add;
		if result < 0 { None } else { Some(result) }
	}

	// return can win or not with this strategy
	fn go_down_without_adding(&self) -> bool { self.current / self.must_sub >= self.need_days }

	// goes until `until`, if can, return Some(is_win), else return None
	fn go_until(&mut self, until: i64, speed: i64) -> Option<bool> {
		if self.current > until {
			let days = (self.current - until) / speed;
			if days >= self.need_days {
				self.current -= self.need_days * speed;
				self.need_days = 0;
				Some(true)
			} else {
				self.need_days -= days;
				self.current -= days * speed;

				// Add one more day if needed
				if self.current > until && self.need_days > 0 && self.current >= speed {
					self.need_days -= 1;
					self.current -= speed;
				}

				if self.current <= until && self.current >= 0 { Some(self.is_win()) } else { None }
			}
		} else {
			// we already here
			Some(self.is_win())
		}
	}

	fn is_win(&self) -> bool { self.need_days <= 0 }
}

fn solve(mut input: Input) -> bool {
	match input.can_add.cmp(&input.must_sub) {
		Ordering::Equal => {
			input.step();
			input.current >= 0
		},
		Ordering::Greater => {
			if let Some(where_can_add) = input.where_can_add() {
				if where_can_add + 1 >= input.must_sub {
					true
				} else if let Some(is_win) = input.go_until(where_can_add, input.must_sub) {
					if is_win {
						return true;
					}
					let mut was = vec![false; input.must_sub as usize];
					loop {
						if input.current < 0 {
							return false;
						}
						if input.current + input.can_add <= input.max {
							if input.current >= 0 {
								let pos = input.current % input.must_sub;
								if was[pos as usize] {
									return true;
								}
								was[pos as usize] = true;
							}
							input.current += input.can_add;
						}
						if let Some(is_win) = input.go_until(where_can_add, input.must_sub) {
							if is_win {
								return true;
							}
						} else {
							return false;
						}
					}
				} else {
					false
				}
			} else {
				input.go_down_without_adding()
			}
		},
		Ordering::Less => {
			if let Some(where_can_add) = input.where_can_add() {
				// Go down without adding until we reach adding point
				if let Some(is_win) = input.go_until(where_can_add, input.must_sub) {
					if is_win {
						return true;
					}
					if let Some(is_win) = input.go_until(0, input.must_sub - input.can_add) { is_win } else { false }
				} else {
					false
				}
			} else {
				input.go_down_without_adding()
			}
		},
	}
}

#[fastio::fastio]
pub fn main() {
	let a = readln!(i64);
	let input = Input { current: a[0] - a[1], max: a[2] - a[1], need_days: a[3], must_sub: a[4], can_add: a[5] };
	if solve(input) {
		println!("Yes");
	} else {
		println!("No");
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/ \/ TESTING CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
//----------------------------------------------------------------------------

impl Input {
	// step for long solve function
	fn step(&mut self) {
		if self.current + self.can_add <= self.max {
			self.current += self.can_add;
		}
		self.current -= self.must_sub;
	}
}

fn long_solve(mut input: Input) -> bool {
	for _ in 0..=input.need_days {
		if input.current < 0 {
			return false;
		}
		input.step();
	}
	true
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn name() {
		use rand::{seq::SliceRandom, Rng};
		let mut rng = rand::thread_rng();
		for _ in 0..200000 {
			let max = rng.gen_range(10, 1000);
			let current = rng.gen_range(0, max);
			let can_add = rng.gen_range(1, 1000);
			let must_sub = rng.gen_range(1, 1000);
			for need_days in 1..1000 {
				let input = Input { current, max, can_add, must_sub, need_days };
				let fast_ans = solve(input.clone());
				let long_ans = long_solve(input.clone());
				if fast_ans != long_ans {
					dbg!(input, fast_ans, long_ans);
					panic!();
				}
			}
		}
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
