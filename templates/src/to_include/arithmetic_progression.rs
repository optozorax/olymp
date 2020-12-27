mod arithmetic_progression {
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
            use super::quadratic_equation::SolvingResult::*;
            let a = self.d as f64;
            let b = 2.0*self.a1 as f64 - self.d as f64;
            let c = -2.0 * sum as f64;
            match super::quadratic_equation::solve(a, b, c) {
                Two(a, _) => a as i64, // TODO think about another?
                One(a) => a as i64,
                Zero => 0,
            }
        }
    }
}
