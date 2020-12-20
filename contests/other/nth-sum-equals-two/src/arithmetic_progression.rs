use crate::quadratic_equation;

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