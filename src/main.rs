#![allow(special_module_name)]

//Making this pub silences dead_code warnings on pub members.
pub mod lib;
mod years;

#[cfg(feature = "default")]
const YEAR: usize = usize::max_value();
#[cfg(all(feature = "year2015", not(feature = "default")))]
const YEAR: usize = 2015;
#[cfg(all(feature = "year2016", not(feature = "default")))]
const YEAR: usize = 2016;
#[cfg(all(feature = "year2017", not(feature = "default")))]
const YEAR: usize = 2017;
#[cfg(all(feature = "year2018", not(feature = "default")))]
const YEAR: usize = 2018;
#[cfg(all(feature = "year2019", not(feature = "default")))]
const YEAR: usize = 2019;
#[cfg(all(feature = "year2020", not(feature = "default")))]
const YEAR: usize = 2020;
#[cfg(all(feature = "year2021", not(feature = "default")))]
const YEAR: usize = 2021;
#[cfg(all(feature = "year2022", not(feature = "default")))]
const YEAR: usize = 2022;
#[cfg(all(feature = "year2023", not(feature = "default")))]
const YEAR: usize = 2023;
#[cfg(all(feature = "year2024", not(feature = "default")))]
const YEAR: usize = 2024;

#[cfg(feature = "default")]
const DAY: usize = usize::max_value();
#[cfg(all(feature = "day1", not(feature = "default")))]
const DAY: usize = 1;
#[cfg(all(feature = "day2", not(feature = "default")))]
const DAY: usize = 2;
#[cfg(all(feature = "day3", not(feature = "default")))]
const DAY: usize = 3;
#[cfg(all(feature = "day4", not(feature = "default")))]
const DAY: usize = 4;
#[cfg(all(feature = "day5", not(feature = "default")))]
const DAY: usize = 5;
#[cfg(all(feature = "day6", not(feature = "default")))]
const DAY: usize = 6;
#[cfg(all(feature = "day7", not(feature = "default")))]
const DAY: usize = 7;
#[cfg(all(feature = "day8", not(feature = "default")))]
const DAY: usize = 8;
#[cfg(all(feature = "day9", not(feature = "default")))]
const DAY: usize = 9;
#[cfg(all(feature = "day10", not(feature = "default")))]
const DAY: usize = 10;
#[cfg(all(feature = "day11", not(feature = "default")))]
const DAY: usize = 11;
#[cfg(all(feature = "day12", not(feature = "default")))]
const DAY: usize = 12;
#[cfg(all(feature = "day13", not(feature = "default")))]
const DAY: usize = 13;
#[cfg(all(feature = "day14", not(feature = "default")))]
const DAY: usize = 14;
#[cfg(all(feature = "day15", not(feature = "default")))]
const DAY: usize = 15;
#[cfg(all(feature = "day16", not(feature = "default")))]
const DAY: usize = 16;
#[cfg(all(feature = "day17", not(feature = "default")))]
const DAY: usize = 17;
#[cfg(all(feature = "day18", not(feature = "default")))]
const DAY: usize = 18;
#[cfg(all(feature = "day19", not(feature = "default")))]
const DAY: usize = 19;
#[cfg(all(feature = "day20", not(feature = "default")))]
const DAY: usize = 20;
#[cfg(all(feature = "day20", not(feature = "default")))]
const DAY: usize = 20;
#[cfg(all(feature = "day21", not(feature = "default")))]
const DAY: usize = 21;
#[cfg(all(feature = "day22", not(feature = "default")))]
const DAY: usize = 22;
#[cfg(all(feature = "day23", not(feature = "default")))]
const DAY: usize = 23;
#[cfg(all(feature = "day24", not(feature = "default")))]
const DAY: usize = 24;
#[cfg(all(feature = "day25", not(feature = "default")))]
const DAY: usize = 25;

fn main() -> anyhow::Result<()> {
	dotenv::dotenv().unwrap();

	if cfg!(feature = "default") {
		unimplemented!("Use --no-default-features and --feature 'yearYYYY dayDD [sample] [part2]'")
	}

	let input = if cfg!(any(feature = "sample", feature = "sample2")) {
		lib::input::sample(YEAR, DAY)?
	} else {
		lib::input::input(YEAR, DAY)?
	};

	println!("{}", years::puzzle(input)?);
	Ok(())
}
