#[derive(Debug)]
struct Robot {
	current_time: i64,
	pos: i64,
	command_no: usize,
	action: RobotEnum,
}

#[derive(Debug)]
enum RobotEnum {
	Waiting,
	Moving { to: i64 },
}

impl Robot {
	fn new() -> Self { Robot { current_time: 0, pos: 0, command_no: 0, action: RobotEnum::Waiting } }

	fn command(&mut self, command_no: usize, t: i64, x: i64, next_t: i64, next_pos: i64, counter: &mut usize) {
		match self.action {
			RobotEnum::Waiting => {
				if self.command_no + 1 == command_no {
					*counter += 1;
				}
				self.current_time = t;
				self.command_no = command_no;
				self.action = RobotEnum::Moving { to: x };
			},
			RobotEnum::Moving { to } => {
				let time_to_next = t - self.current_time;
				let move_time = (self.pos - to).abs();
				if move_time > time_to_next {
					let direction = (to - self.pos).signum();
					let pos_at_time_t = self.pos + time_to_next * direction;
					let time_to_next_next = next_t - self.current_time;
					if time_to_next_next > move_time {
						if ((pos_at_time_t > to) && (to <= x && x <= pos_at_time_t))
							|| ((pos_at_time_t <= to) && (pos_at_time_t <= x && x <= to))
						{
							*counter += 1;
						}
					} else {
						let pos_at_time_next_t = self.pos + time_to_next_next * direction;
						if ((pos_at_time_t > pos_at_time_next_t) && (pos_at_time_next_t <= x && x <= pos_at_time_t))
							|| ((pos_at_time_t <= pos_at_time_next_t)
								&& (pos_at_time_t <= x && x <= pos_at_time_next_t))
						{
							*counter += 1;
						}
					}
					self.current_time += time_to_next;
					self.pos = pos_at_time_t;
					self.action = RobotEnum::Moving { to };
				} else {
					self.current_time += move_time;
					self.pos = to;
					self.action = RobotEnum::Waiting;
					self.command(command_no, t, x, next_t, next_pos, counter);
				}
			},
		}
	}
}

#[fastio::fastio]
pub fn main() {
	let t = read!(usize);
	for _ in 0..t {
		let n = read!(usize);
		let mut actions = Vec::new();
		for _ in 0..n {
			let t = read!(i64);
			let x = read!(i64);
			actions.push((t, x));
		}
		actions.push((3_000_000_100, 3_000_000_100));

		let mut robot = Robot::new();
		let mut counter = 0;
		for (i, commands) in actions.windows(2).enumerate() {
			robot.command(i + 1, commands[0].0, commands[0].1, commands[1].0, commands[1].1, &mut counter);
		}
		if robot.command_no == n {
			counter += 1;
		}
		println!("{}", counter - 1);
	}
}

//----------------------------------------------------------------------------
// \/ \/ \/ \/ \/ \/ \/ \/ \/  PRE-WRITTEN CODE \/ \/ \/ \/ \/ \/ \/ \/ \/ \/
// Source: https://github.com/optozorax/olymp/tree/master/templates ----------
//----------------------------------------------------------------------------

include!("../../../../templates/src/to_include/imports.rs");
include!("../../../../templates/src/to_include/scanner.rs");
