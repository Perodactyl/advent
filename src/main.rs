#![allow(special_module_name)]

use clap::{Parser, ValueEnum};

//Making this pub silences dead_code warnings on pub members.
pub mod lib;
mod years;

#[derive(ValueEnum, Default, Debug, Clone, Copy)]
enum InputOption {
	Sample,
	#[default]
	Input
}

#[derive(Parser)]
struct Args {
	#[arg(short, long, env)]
	year: usize,
	#[arg(short, long, env)]
	day: usize,
	#[arg(short, long, value_enum, default_value_t, env)]
	source: InputOption,
}

fn main() -> anyhow::Result<()> {
	dotenv::dotenv().unwrap();
	let args = Args::parse();

	let input = match args.source {
		InputOption::Sample => lib::input::sample(args.year, args.day).unwrap(),
		InputOption::Input => lib::input::input(args.year, args.day).unwrap(),
	};

	println!("{}", years::puzzle(args.year, args.day, input)?);
	Ok(())
}
