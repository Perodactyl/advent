include!("generated_list.rs");

#[cfg(feature = "default")]
mod day {
	pub fn main(_: String) -> anyhow::Result<String> {
		unimplemented!("Do not use default features!");
	}
}

pub fn puzzle(input: String) -> anyhow::Result<String> {
	day::main(input)
}
