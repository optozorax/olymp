use crate::arithmetic_progression::*;

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