#[cfg(feature = "year2024")]
#[allow(dead_code)]
mod year2024;
#[cfg(all(feature = "year2024", not(feature = "default")))]
use year2024 as year;

#[cfg(feature = "default")]
mod year {
	pub fn puzzle(_: String) -> anyhow::Result<String> {
		unimplemented!("Do not use default features!");
	}
}

pub fn puzzle(input: String) -> anyhow::Result<String> {
	year::puzzle(input)
}
