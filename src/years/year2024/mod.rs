#[cfg(feature = "day6")]
mod day6;
#[cfg(all(feature = "day6", not(feature = "default")))]
use day6 as day;
#[cfg(feature = "day7")]
mod day7;
#[cfg(all(feature = "day7", not(feature = "default")))]
use day7 as day;
#[cfg(feature = "day11")]
mod day11;
#[cfg(all(feature = "day11", not(feature = "default")))]
use day11 as day;
#[cfg(feature = "day12")]
mod day12;
#[cfg(all(feature = "day12", not(feature = "default")))]
use day12 as day;
#[cfg(feature = "day14")]
mod day14;
#[cfg(all(feature = "day14", not(feature = "default")))]
use day14 as day;
#[cfg(feature = "day15")]
mod day15;
#[cfg(all(feature = "day15", not(feature = "default")))]
use day15 as day;

#[cfg(feature = "default")]
mod day {
	pub fn main(_: String) -> anyhow::Result<String> {
		unimplemented!("Do not use default features!");
	}
}

pub fn puzzle(input: String) -> anyhow::Result<String> {
	day::main(input)
}
