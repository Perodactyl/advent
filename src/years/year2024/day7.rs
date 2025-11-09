use crate::lib::prelude::*;

#[derive(VariantArray, Default, Debug, Clone, Copy, Display, PartialEq, Eq)]
enum Operation {
	#[default]
	#[strum(to_string = "+")]
	Add,
	#[strum(to_string = "*")]
	Multiply,
	#[strum(to_string = "||")]
	Concatenate,
}

pub fn main(input: String) -> Result<String> {
	let mut answer = 0;
	let lines = input.lines().count();
	init_progress_bar_with_eta(lines);

	for line in input.lines() {
		let Some((target, rhs)) = line.split_once(':') else { panic!("{line} is malformed") };
		let target: u64 = target.parse().unwrap();
		let parts: Vec<u64> = rhs.trim().split_whitespace().map(|v| v.parse().unwrap()).collect();

		let mut state = vec![0; parts.len() - 1];
		let mut solvable = false;
		'test_states: loop {
			//Test the validity of this state
			let mut sum = parts[0];
			for i in 0..state.len() {
				let operation = Operation::VARIANTS[state[i]];
				let num = parts[i+1];

				match operation {
					Operation::Add => sum += num,
					Operation::Multiply => sum *= num,
					Operation::Concatenate => {
						sum = format!("{sum}{num}").parse().unwrap();
					},
				}
			}
			if sum == target {
				solvable = true;
				print_progress_bar_info(
					"Found",
					&format!("{target}: {}{}", parts[0], parts
						.iter()
						.skip(1)
						.enumerate()
						.map(|(i,p)| format!(" {} {p}", Operation::VARIANTS[state[i]]))
						.collect::<Vec<_>>()
						.join(" ")),
					Color::Green,
					Style::Bold
				);
				inc_progress_bar();
				break
			};

			//Increment the state
			state[0] += 1;
			for i in 0..state.len() {
				if state[i] >= Operation::VARIANTS.len() {
					state[i] = 0;
					if i != state.len()-1 { state[i+1] += 1 }
					else {
						print_progress_bar_info(
							"Failed",
							&format!("{target}: {}", parts.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(" ")),
							Color::Red,
							Style::Bold
						);
						inc_progress_bar();
						break 'test_states
					}
				}
			}
		}
		if solvable { answer += target };
	}
	finalize_progress_bar();

	Ok(format!("{answer}"))
}
