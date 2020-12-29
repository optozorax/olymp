/* Вторая версия моей программы, где я пытался найти брут-форсом то, о чём и не догадывался. */

struct InputForAbstractOp {
	x: usize,
	y: usize,
	n: usize,
	value: u16,
}

struct OutputForAbstractOp {
	x: usize,
	y: usize,
	value: u16,
}

fn op_abstract<F>(vec: &[Vec<u16>], f: F) -> Vec<Vec<u16>>
where F: Fn(InputForAbstractOp) -> OutputForAbstractOp {
	let n = vec.len();
	let mut result = vec![vec![0u16; n]; n];
	for (y, line) in vec.iter().enumerate() {
		for (x, value) in line.iter().copied().enumerate() {
			let input = InputForAbstractOp { x, y, n, value };
			let output = f(input);
			result[output.y][output.x] = output.value;
		}
	}
	result
}

fn op_r_many(vec: &[Vec<u16>], count: usize) -> Vec<Vec<u16>> {
	op_abstract(vec, |InputForAbstractOp { x, y, n, value }| OutputForAbstractOp { x: (x + count) % n, y, value })
}

fn op_r(vec: &[Vec<u16>]) -> Vec<Vec<u16>> {
	op_abstract(vec, |InputForAbstractOp { x, y, n, value }| OutputForAbstractOp { x: (x + 1) % n, y, value })
}

fn op_l(vec: &[Vec<u16>]) -> Vec<Vec<u16>> {
	op_abstract(vec, |InputForAbstractOp { x, y, n, value }| OutputForAbstractOp { x: (n + x - 1) % n, y, value })
}

fn op_d(vec: &[Vec<u16>]) -> Vec<Vec<u16>> {
	op_abstract(vec, |InputForAbstractOp { x, y, n, value }| OutputForAbstractOp { x, y: (y + 1) % n, value })
}

fn op_u(vec: &[Vec<u16>]) -> Vec<Vec<u16>> {
	op_abstract(vec, |InputForAbstractOp { x, y, n, value }| OutputForAbstractOp { x, y: (n + y - 1) % n, value })
}

fn op_i(vec: &[Vec<u16>]) -> Vec<Vec<u16>> {
	op_abstract(vec, |InputForAbstractOp { x, y, n: _, value }| OutputForAbstractOp {
		x: value as usize - 1,
		y,
		value: x as u16 + 1,
	})
}

fn op_c(vec: &[Vec<u16>]) -> Vec<Vec<u16>> {
	op_abstract(vec, |InputForAbstractOp { x, y, n: _, value }| OutputForAbstractOp {
		x,
		y: value as usize - 1,
		value: y as u16 + 1,
	})
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Action {
	R,
	L,
	D,
	U,
	I,
	C,
}

impl TryFrom<u8> for Action {
	type Error = ();

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			b'L' => Ok(L),
			b'R' => Ok(R),
			b'D' => Ok(D),
			b'U' => Ok(U),
			b'I' => Ok(I),
			b'C' => Ok(C),
			_ => Err(()),
		}
	}
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum State {
	S0,
	SI,
	SC,
}

use Action::*;
use State::*;

const ALL_ACTIONS: [Action; 6] = [R, L, D, U, I, C];

fn op(vec: &[Vec<u16>], op: Action) -> Vec<Vec<u16>> {
	match op {
		L => op_l(vec),
		R => op_r(vec),
		D => op_d(vec),
		U => op_u(vec),
		I => op_i(vec),
		C => op_c(vec),
	}
}

fn get_current_state(input: &[Vec<u16>]) -> State {
	if input == &op_c(input)[..] {
		SI
	} else if input == &op_i(input)[..] {
		SC
	} else {
		S0
	}
}

fn get_current_pos(input: &[Vec<u16>], state: State) -> usize {
	let forward = input[0][0] as usize - 1;
	let reverse = (input.len() - forward) % input.len();
	match state {
		S0 => reverse,
		SI => reverse,
		SC => forward,
	}
}

fn gen_default_matrix(n: u16) -> Vec<Vec<u16>> {
	(0..n)
		.map(|x| (0..n).map(|y| (x + y) % n + 1).collect::<Vec<u16>>())
		.collect()
}

#[derive(Clone, Copy)]
enum HowChange {
	Step(isize),
	Change(State),
	ChangeReverse(State),
	Nothing,
}
use HowChange::*;

struct Solver {
	graph_map: BTreeMap<State, BTreeMap<Action, HowChange>>,
}

impl Solver {
	fn new() -> Self {
		#[rustfmt::skip]
        let graph = [
            (SC, [(R, Step(1)), (L, Step(-1)), (D, Step(-1)), (U,  Step(1)), (I, Nothing),           (C, Change(S0))]),
            (S0, [(R, Step(1)), (L, Step(-1)), (D,  Step(1)), (U, Step(-1)), (I, ChangeReverse(SI)), (C, Change(SC))]),
            (SI, [(R, Step(1)), (L, Step(-1)), (D, Step(-1)), (U,  Step(1)), (I, ChangeReverse(S0)), (C, Nothing)   ]),
        ];

		Self {
			graph_map: graph
				.iter()
				.map(|(state, actions)| (*state, actions.iter().copied().collect::<BTreeMap<Action, HowChange>>()))
				.collect::<BTreeMap<State, BTreeMap<Action, HowChange>>>(),
		}
	}

	fn solve_fast(&self, matrix: &[Vec<u16>], commands: &[Action]) -> Vec<Vec<u16>> {
		// dbg!("fast:");
		let n = matrix.len();
		let mut current_state = get_current_state(&matrix);
		let mut current_pos: usize = get_current_pos(&matrix, current_state);
		for command in commands {
			// eprintln!("{:?} {:?} {}", command, current_state, current_pos);
			match self.graph_map.get(&current_state).unwrap().get(&command).unwrap() {
				Step(count) => current_pos = (current_pos + (n as isize + count) as usize) % n,
				Change(state) => current_state = *state,
				ChangeReverse(state) => {
					current_state = *state;
					current_pos = (n - current_pos) % n;
				},
				Nothing => {},
			}
		}
		// eprintln!("{:?} {}", current_state, current_pos);

		let result = gen_default_matrix(n as u16);
		let result = match current_state {
			SI => op_i(&result),
			S0 => result,
			SC => op_c(&result),
		};
		op_r_many(&result, current_pos)
	}

	fn solve_slow(&self, matrix: &[Vec<u16>], commands: &[Action]) -> Vec<Vec<u16>> {
		// dbg!();
		// dbg!("slow:");
		// let mut current = matrix.to_vec();
		// for command in commands {
		//     let current_state = get_current_state(&current);
		//     let current_pos: usize = get_current_pos(&current, current_state);
		//     eprintln!("{:?} {:?} {}", command, current_state, current_pos);
		//     current = op(&current, *command);
		// }
		// let current_state = get_current_state(&current);
		// let current_pos: usize = get_current_pos(&current, current_state);
		// eprintln!("{:?} {}", current_state, current_pos);

		let result = commands.iter().fold(matrix.to_vec(), |acc, c| op(&acc, *c));
		// let current_state = get_current_state(&result);
		// let current_pos: usize = get_current_pos(&result, current_state);
		// eprintln!("result {:?} {}", current_state, current_pos);
		result
	}
}

#[fastio::fastio]
pub fn main() {
	// `RUSTFLAGS="--cfg print_graph" cargo run`
	#[cfg(print_graph)]
	{
		let n = 10;

		let a0 = gen_default_matrix(n as u16);
		let ac = op_c(&a0);
		let ai = op_i(&a0);

		let mut nodes: BTreeMap<Vec<Vec<u16>>, String> = BTreeMap::new();
		for i in 0..n {
			let a0i = op_r_many(&a0, i);
			let aci = op_r_many(&ac, i);
			let aii = op_r_many(&ai, i);

			nodes.insert(aci.clone(), format!("c{}", i));
			nodes.insert(aii.clone(), format!("i{}", i));
			nodes.insert(a0i.clone(), format!("{}", i));

			assert_eq!(get_current_state(&a0i), S0);
			assert_eq!(get_current_state(&aci), SC);
			assert_eq!(get_current_state(&aii), SI);

			assert_eq!(get_current_pos(&a0i, S0), i);
			assert_eq!(get_current_pos(&aci, SC), i);
			assert_eq!(get_current_pos(&aii, SI), i);
		}

		let mut edges_clusters: BTreeSet<(String, String, Action)> = BTreeSet::new();
		let mut edges_full: BTreeSet<(String, String, Action)> = BTreeSet::new();
		for (a, name) in &nodes {
			for c in &ALL_ACTIONS {
				let edge = (name.clone(), nodes.get(&op(a, *c)).unwrap().clone(), *c);
				if *c != I && *c != C {
					edges_clusters.insert(edge.clone());
				}
				if *c != D && *c != U && ((name.starts_with('i') && *c != R) || (!name.starts_with('i') && *c != L)) {
					edges_full.insert(edge);
				}
			}
		}

		// Print full graph
		println!("digraph G {{");
		println!("  subgraph cluster_0 {{");
		println!("    label=\"State C\";");
		println!("    {};", (0..n).map(|x| format!("c{}", x)).collect::<Vec<_>>().join(", "));
		println!("  }}");
		println!("  subgraph cluster_1 {{");
		println!("    label=\"State 0\";");
		println!("    {};", (0..n).map(|x| format!("{}", x)).collect::<Vec<_>>().join(", "));
		println!("  }}");
		println!("  subgraph cluster_2 {{");
		println!("    label=\"State I\";");
		println!("    {};", (0..n).map(|x| format!("i{}", x)).collect::<Vec<_>>().join(", "));
		println!("  }}");

		for i in edges_full {
			println!(
				"  {from} -> {to} [label=<<font color=\"{color}\">{label:?}</font>>,color=\"{color}\"];",
				from = i.0,
				to = i.1,
				label = i.2,
				color = match i.2 {
					R => "red",
					L => "blue",

					U => "orange",
					D => "green",

					I => "black",
					C => "gray",
				},
			);
		}
		println!("}}");

		// Print clusters
		println!("digraph G {{");
		for i in edges_clusters {
			println!(
				"  {from} -> {to} [label=<<font color=\"{color}\">{label:?}</font>>,color=\"{color}\"];",
				from = i.0,
				to = i.1,
				label = i.2,
				color = match i.2 {
					R => "red",
					L => "blue",

					U => "orange",
					D => "green",

					I => "black",
					C => "gray",
				},
			);
		}
		println!("}}");

		writer.flush().unwrap();
	}

	#[cfg(not(print_graph))]
	{
		let solver = Solver::new();

		let t = read!(usize);
		for _ in 0..t {
			let n = read!(usize);
			let _m = read!(i64);
			let matrix = {
				let mut matrix = Vec::new();
				for _ in 0..n {
					let line = readln!(u16);
					matrix.push(line);
				}
				matrix
			};

			let s = bytes!()
				.into_iter()
				.map(|c| Action::try_from(c).unwrap())
				.collect::<Vec<_>>();

			let result = solver.solve_fast(&matrix, &s);
			// let result = solver.solve_slow(&matrix, &s);

			println!("{}", Lines(result.into_iter().map(SpaceVec).collect()));
		}
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/ \/ TESTING CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
//----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn name() {
		use rand::Rng;
		let mut rng = rand::thread_rng();
		let solver = Solver::new();
		for size in 1..20 {
			dbg!(size);
			for _ in 0..200000 {
				let commands_size = rng.gen_range(1, 35);
				let commands = (0..commands_size)
					.map(|_| match rng.gen_range(0, 6) {
						0 => R,
						1 => L,
						2 => D,
						3 => U,
						4 => C,
						_ => I,
					})
					.collect::<Vec<_>>();
				let matrix = gen_default_matrix(size);
				let fast_ans = solver.solve_fast(&matrix, &commands);
				let long_ans = solver.solve_slow(&matrix, &commands);
				if fast_ans != long_ans {
					let matrix = Lines(matrix.into_iter().map(SpaceVec).collect());
					let fast_ans = Lines(fast_ans.into_iter().map(SpaceVec).collect());
					let long_ans = Lines(long_ans.into_iter().map(SpaceVec).collect());
					eprintln!("commands: {}\n", commands.into_iter().map(|x| format!("{:?}", x)).collect::<String>());
					eprintln!("matrix:\n{}\n", matrix);
					eprintln!("fast_ans:\n{}\n", fast_ans);
					eprintln!("long_ans:\n{}\n", long_ans);
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
include!("../../../../templates/src/to_include/display/lines.rs");
include!("../../../../templates/src/to_include/display/space_vec.rs");
