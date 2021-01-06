#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
enum DoneResult {
	OneWinner(Player),
	Tie,
}

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
enum GameResult {
	Done(DoneResult),
	NotDone,
}

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
enum Player {
	First,
	Second,
}

trait OptimalGame: Clone {
	type StepInfo;
	type CurrentStepsIter: Iterator<Item = Self::StepInfo>;

	fn iter_current_steps(&self) -> Self::CurrentStepsIter;
	fn step(&mut self, info: Self::StepInfo);
	fn who_wins(&self) -> GameResult;
	fn is_done(&self) -> bool;
	fn current_player(&self) -> Player;
}

#[derive(Default)]
struct BruteForceResult {
	first_wins_count: usize,
	second_wins_count: usize,
	tie_count: usize,
}

impl Add for BruteForceResult {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		Self {
			first_wins_count: self.first_wins_count + other.first_wins_count,
			second_wins_count: self.second_wins_count + other.second_wins_count,
			tie_count: self.tie_count + other.tie_count,
		}
	}
}

impl AddAssign for BruteForceResult {
	fn add_assign(&mut self, other: Self) {
		self.first_wins_count += other.first_wins_count;
		self.second_wins_count += other.second_wins_count;
		self.tie_count += other.tie_count;
	}
}

impl From<DoneResult> for BruteForceResult {
	fn from(input: DoneResult) -> Self {
		let mut result = BruteForceResult::default();
		match input {
			DoneResult::OneWinner(Player::First) => result.first_wins_count += 1,
			DoneResult::OneWinner(Player::Second) => result.second_wins_count += 1,
			DoneResult::Tie => result.tie_count += 1,
		}
		result
	}
}

fn play_brute_force<G: OptimalGame>(g: G) -> BruteForceResult {
	let mut result = BruteForceResult::default();

	for step_info in g.iter_current_steps() {
		let mut gcopy = g.clone();
		gcopy.step(step_info);
		result += match gcopy.who_wins() {
			GameResult::Done(done_result) => done_result.into(),
			GameResult::NotDone => play_brute_force(gcopy),
		};
	}
	result
}

fn play_optimally_slow<G: OptimalGame + Debug>(init_game: G) -> Vec<G> {
	let mut current_games = vec![init_game];

	while !current_games.iter().all(|x| x.is_done()) {
		let mut to_push = vec![];
		for current_game in current_games {
			if !current_game.is_done() {
				let mut optimal_steps = current_game
					.iter_current_steps()
					.map(|step_info| {
						let mut gcopy = current_game.clone();
						let player = gcopy.current_player();
						gcopy.step(step_info);
						let brute_force_result = match gcopy.who_wins() {
							GameResult::Done(done_result) => done_result.into(),
							GameResult::NotDone => play_brute_force(gcopy.clone()),
						};
						if player == Player::First {
							((brute_force_result.first_wins_count, brute_force_result.tie_count), gcopy)
						} else {
							((brute_force_result.second_wins_count, brute_force_result.tie_count), gcopy)
						}
					})
					.collect::<Vec<_>>();

				optimal_steps.sort_unstable_by_key(|x| Reverse(x.0));

				let first = optimal_steps[0].0;
				for (result, game) in optimal_steps {
					if result == first {
						to_push.push(game);
					}
				}
			} else {
				to_push.push(current_game);
			}
		}
		current_games = to_push;
	}

	assert_all_games_has_same_result(&current_games);

	current_games
}

fn assert_all_games_has_same_result<G: OptimalGame + Debug>(games: &[G]) {
	let first = games[0].who_wins();
	for (index, game) in games.iter().enumerate() {
		if game.who_wins() != first {
			dbg!(first, &games[0], index, game);
			panic!("Not all games has same result");
		}
	}
}
