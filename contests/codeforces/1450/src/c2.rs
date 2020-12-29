#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Copy)]
pub enum Elem {
	None,
	X,
	O,
}

pub fn invert(e: Elem) -> Elem {
	use Elem::*;
	match e {
		None => None,
		X => O,
		O => X,
	}
}

impl Display for Elem {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		use Elem::*;
		match self {
			None => write!(f, "."),
			X => write!(f, "X"),
			O => write!(f, "O"),
		}
	}
}

impl From<u8> for Elem {
	fn from(s: u8) -> Self {
		use Elem::*;
		match s {
			b'.' => None,
			b'X' => X,
			b'O' => O,
			_ => unreachable!(),
		}
	}
}

pub fn check(field: &[Vec<Elem>]) -> bool {
	let n = field.len();
	for i in 0..n {
		for j in 0..n {
			let current = field[i][j];
			if current != Elem::None
				&& ((i + 2 < n && field[i + 1][j] == current && field[i + 2][j] == current)
					|| (j + 2 < n && field[i][j + 1] == current && field[i][j + 2] == current))
			{
				return false;
			}
		}
	}
	true
}

pub fn count_xo(field: &[Vec<Elem>]) -> usize {
	field
		.iter()
		.map(|line| line.iter().map(|x| (*x != Elem::None) as usize).sum::<usize>())
		.sum()
}

fn solve(field: Vec<Vec<Elem>>) -> Option<Vec<Vec<Elem>>> {
	let all_xo_count: usize = count_xo(&field);
	for x_start in 0..3 {
		for y_start in 0..3 {
			let mut field = field.clone();
			let mut count = 0;
			for (i, line) in field.iter_mut().enumerate() {
				for (j, current) in line.iter_mut().enumerate() {
					if *current != Elem::None
						&& ((*current == Elem::X && (i + j) % 3 == x_start)
							|| (*current == Elem::O && (i + j) % 3 == y_start))
					{
						*current = invert(*current);
						count += 1;
					}
				}
			}
			if count <= all_xo_count / 3 && check(&field) {
				return Some(field);
			}
		}
	}
	None
}

#[fastio::fastio]
pub fn main() {
	let t = read!(usize);
	for _ in 0..t {
		let n = read!(usize);
		let mut field = Vec::with_capacity(n);
		for _ in 0..n {
			let a = bytes!().into_iter().map(Elem::from).collect::<Vec<_>>();
			field.push(a);
		}
		let field = solve(field).unwrap();

		println!("{}", field.into_iter().map(|x| x.into_iter().joined_by("")).joined_by('\n'));
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
include!("../../../../templates/src/to_include/display/joined_by.rs");
