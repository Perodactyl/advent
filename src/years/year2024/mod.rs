mod day7;

pub fn puzzle(day: usize, input: String) -> anyhow::Result<String> {
	match day {
		7 => day7::main(input),
		d => panic!("Day {d} is not existant")
	}
}
