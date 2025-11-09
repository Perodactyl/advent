mod year2024;

pub fn puzzle(year: usize, day: usize, input: String) -> anyhow::Result<String> {
	match year {
		2024 => year2024::puzzle(day, input),
		y => panic!("Year {y} is not existant."),
	}
}
