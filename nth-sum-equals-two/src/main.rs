mod quadratic_equation {
	#[derive(Debug, Clone)]
	pub enum SolvingResult {
		Two(f64, f64),
		One(f64),
		Zero,
	}

	pub fn solve(a: f64, b: f64, c: f64) -> SolvingResult {
		use SolvingResult::*;
		let d = b*b - 4.0*a*c;
		if d < 0.0 {
			Zero
		} else if d.abs() < 1e-9 {
			One(-b/(2.0*a))
		} else {
			let sq_d = d.sqrt();
			let x1 = (-b + sq_d)/(2.0*a);
			let x2 = (-b - sq_d)/(2.0*a);
			Two(
				x1.max(x2),
				x1.min(x2),
			)
		}
	}
}

mod arithmetic_progression {
	use super::quadratic_equation;

	pub struct ArithmeticProgresion {
		a1: i64,
		d: i64,
	}

	impl ArithmeticProgresion {
		pub fn new_canonical() -> Self {
			Self {
				a1: 1,
				d: 1,
			}
		}

		pub fn new(a1: i64, d: i64) -> Self {
			Self {
				a1,
				d,
			}
		}

		/// N-th elemnt of arithmetic progression
		pub fn nth(&self, n: i64) -> i64 {
			self.a1 + self.d * n
		}

		/// Same as `(0..n).map(|x| self.nth(x)).sum()`, but `O(1)` complexity, optimized by formula.
		pub fn sum_to(&self, n: i64) -> i64 {
			(2*self.a1 + self.d*(n-1))*n/2
		}

		/// For this sum returns such `n` that: `self.sum_to(n) <= sum < self.sum_to(n+1)`.
		pub fn from_sum(&self, sum: i64) -> i64 {
			use quadratic_equation::SolvingResult::*;
			let a = self.d as f64;
			let b = 2.0*self.a1 as f64 - self.d as f64;
			let c = -2.0 * sum as f64;
			match quadratic_equation::solve(a, b, c) {
				Two(a, _) => a as i64, // TODO think about another?
				One(a) => a as i64,
				Zero => 0,
			}
		}
	}

	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn canonical() {
			let ar = ArithmeticProgresion::new_canonical();

			assert_eq!(ar.sum_to(0), 0);
			assert_eq!(ar.sum_to(1), 1);
			assert_eq!(ar.sum_to(2), 3);
			assert_eq!(ar.sum_to(3), 6);
			assert_eq!(ar.sum_to(100), 5050);

			assert_eq!(ar.from_sum(0), 0);
			assert_eq!(ar.from_sum(1), 1);
			assert_eq!(ar.from_sum(2), 1);
			assert_eq!(ar.from_sum(3), 2);
			assert_eq!(ar.from_sum(4), 2);
			assert_eq!(ar.from_sum(5), 2);
			assert_eq!(ar.from_sum(6), 3);
			assert_eq!(ar.from_sum(5050), 100);
			assert_eq!(ar.from_sum(5055), 100);
			assert_eq!(ar.from_sum(5100), 100);

			for sum in 0..1000 {
				let n = ar.from_sum(sum);
				assert!(ar.sum_to(n) <= sum);
				assert!(sum < ar.sum_to(n+1));
			}
		}

		#[test]
		fn complex() {
			let ar = ArithmeticProgresion::new(5, 10);

			assert_eq!(ar.nth(0), 5);
			assert_eq!(ar.nth(1), 15);
			assert_eq!(ar.nth(2), 25);
			assert_eq!(ar.nth(3), 35);		

			assert_eq!(ar.sum_to(0), 0);
			assert_eq!(ar.sum_to(1), ar.nth(0));
			assert_eq!(ar.sum_to(2), ar.nth(0) + ar.nth(1));
			assert_eq!(ar.sum_to(10), (0..10).map(|x| ar.nth(x)).sum());

			assert_eq!(ar.from_sum(0), 0);
			assert_eq!(ar.from_sum(4), 0);
			assert_eq!(ar.from_sum(5), 1);
			assert_eq!(ar.from_sum(10), 1);
			assert_eq!(ar.from_sum(16), 1);

			for sum in 0..1000 {
				let n = ar.from_sum(sum);
				assert!(ar.sum_to(n) <= sum);
				assert!(sum < ar.sum_to(n+1));
			}
		}
	}
}

mod main_task {
	use super::arithmetic_progression::*;

	#[derive(Debug)]
	pub enum Answer {
		Ones {
			size: i64,
			index: i64,
		},
		Two {
			size: i64,
		}
	}

	pub fn calc(n: i64) -> Answer {
		let ar = ArithmeticProgresion::new_canonical();
		let size = ar.from_sum(n)+1;
		let last_start_pos = ar.sum_to(size-1);
		let index = n - last_start_pos;
		let length = ar.nth(size-1);
		if index+1 == length {
			Answer::Two { size }
		} else {
			Answer::Ones { size, index }
		}
	}

	use std::fmt;
	use std::fmt::{Display, Formatter};

	impl Display for Answer {
		fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
			match self {
				Answer::Ones { size, index } => {
					write!(f, "1")?;
					for i in (0..(*size-1)).rev() {
						if i == *index {
							write!(f, "1")?;	
						} else {
							write!(f, "0")?;
						}
					}
				},
				Answer::Two { size } => {
					write!(f, "2")?;
					for _ in 0..(*size-1) {
						write!(f, "0")?;
					}
				}
			}
			Ok(())
		}
	}

	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn main() {
			fn print_answer(n: i64) -> String {
				format!("{}", calc(n))
			}

			assert_eq!("2", print_answer(0));
			assert_eq!("11", print_answer(1));
			assert_eq!("20", print_answer(2));
			assert_eq!("101", print_answer(3));
			assert_eq!("110", print_answer(4));
			assert_eq!("200", print_answer(5));
			assert_eq!("1001", print_answer(6));
			assert_eq!("1010", print_answer(7));
			assert_eq!("1100", print_answer(8));
			assert_eq!("2000", print_answer(9));
			assert_eq!("10001", print_answer(10));
		}
	}
}

fn main() {
	let now = std::time::Instant::now();
	println!("Answer: {}", main_task::calc(100));
	println!("Elapsed time: {:?}", now.elapsed());
}