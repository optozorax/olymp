#[derive(Default, Clone, Debug)]
struct StringActions {
	free: bool,
	letter_determined: BTreeMap<u8, BTreeSet<LetterDetermined>>,
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Ord, Eq)]
enum LetterDetermined {
	OneAction,
	TwoActions(u8),
	ThreeActions(u8, u8),
}

fn solve<B: Fn(u8) -> u8>(input: &[u8], better: B) -> Vec<u8> {
	use LetterDetermined::*;
	let mut actions: Vec<StringActions> = vec![StringActions::default(); input.len() + 2];
	actions[0].free = true;
	let mut result = Vec::new();
	for i in 0..input.len() {
		if actions[i].free {
			let current_letter = input[i];
			// B
			actions[i]
				.letter_determined
				.entry(better(current_letter))
				.or_insert_with(BTreeSet::new)
				.insert(OneAction);
			if i + 1 < input.len() {
				let next_letter = input[i + 1];
				// BL
				actions[i]
					.letter_determined
					.entry(next_letter)
					.or_insert_with(BTreeSet::new)
					.insert(TwoActions(better(current_letter)));
				// RB
				actions[i]
					.letter_determined
					.entry(better(next_letter))
					.or_insert_with(BTreeSet::new)
					.insert(TwoActions(current_letter));
				if i + 2 < input.len() {
					let next_next_letter = input[i + 2];
					// BRL
					actions[i]
						.letter_determined
						.entry(next_next_letter)
						.or_insert_with(BTreeSet::new)
						.insert(ThreeActions(better(current_letter), next_letter));
				}
			}
		}
		let mut current = BTreeMap::new();
		std::mem::swap(&mut current, &mut actions[i].letter_determined);
		let (best_letter, best_determined) = current.into_iter().next().unwrap();
		result.push(best_letter);
		for determined in best_determined {
			match determined {
				OneAction => actions[i + 1].free = true,
				TwoActions(letter) => {
					actions[i + 1]
						.letter_determined
						.entry(letter)
						.or_insert_with(BTreeSet::new)
						.insert(OneAction);
					if i + 2 < input.len() {
						let next_letter = input[i + 2];
						// ?L
						actions[i + 1]
							.letter_determined
							.entry(next_letter)
							.or_insert_with(BTreeSet::new)
							.insert(TwoActions(letter));
						if i + 3 < input.len() {
							let next_next_letter = input[i + 3];
							// ?RL
							actions[i + 1]
								.letter_determined
								.entry(next_next_letter)
								.or_insert_with(BTreeSet::new)
								.insert(ThreeActions(letter, next_letter));
						}
					}
				},
				ThreeActions(letter, next_letter) => {
					actions[i + 1]
						.letter_determined
						.entry(letter)
						.or_insert_with(BTreeSet::new)
						.insert(TwoActions(next_letter));
				},
			}
		}
	}
	result
}

#[fastio::fastio]
pub fn main() {
	let t = read!(usize);
	for _ in 0..t {
		let _n = read!(i64);
		let k = read!(u8);
		let a = bytes!().into_iter().map(|c| c - b'a').collect::<Vec<u8>>();
		let better = |i: u8| -> u8 {
			if i == 0 {
				return i;
			}
			if i + 1 == k {
				return 0;
			}
			i - 1
		};
		let answer = solve(&a, better)
			.into_iter()
			.map(|x| char::from(x + b'a'))
			.collect::<String>();
		println!("{}", answer);
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
