mod day6;
mod day7;
mod day11;

pub fn puzzle(day: usize, input: String) -> anyhow::Result<String> {
	match day {
		6  => day6::main(input),
		7  => day7::main(input),
		11 => day11::main(input),
		d => panic!("Day {d} is not existant")
	}
}
