include!("generated_list.rs");

#[cfg(feature = "default")]
mod year {
	pub fn puzzle(_: String) -> anyhow::Result<String> {
		unimplemented!("Do not use default features!");
	}
}

pub fn puzzle(input: String) -> anyhow::Result<String> {
	year::puzzle(input)
}
