use std::fs;

// This file is very messy and does not follow best practice (modifies src/).
// I don't care. It works.

fn main() {
	let mut year_list = String::new();
	println!("cargo::rerun-if-changed=src/years");
	for entry in fs::read_dir("src/years").unwrap() {
		let Ok(entry) = entry else { continue };
		if entry.file_type().unwrap().is_dir() {
			let year_name = entry.file_name();
			let year_name = year_name.to_string_lossy();
			println!("cargo::rerun-if-changed=src/years/{year_name}");
			year_list.push_str(&format!("#[cfg(feature = \"{0}\")] #[allow(dead_code)] mod {0};\n#[cfg(all(feature = \"{0}\", not(feature = \"default\")))] use {0} as year;\n", year_name));

			let mut day_list = vec![];
			for day_entry in fs::read_dir(entry.path()).unwrap() {
				let Ok(day_entry) = day_entry else { continue };
				let name = day_entry.file_name();
				let name = name.to_string_lossy();

				if name.starts_with("day") && name.ends_with(".rs") {
					let number: u8 = name[3..name.len()-3].parse().unwrap();
					day_list.push(number);
					println!("cargo::rerun-if-changed=src/years/{year_name}/day{number}.rs");
				}
			}

			day_list.sort();
			let day_list_str = day_list.into_iter().map(|n| format!("#[cfg(feature = \"day{n}\")] mod day{n};\n#[cfg(all(feature = \"day{n}\", not(feature = \"default\")))] use day{n} as day;")).collect::<Vec<String>>().join("\n");

			fs::write(format!("src/years/{year_name}/generated_list.rs"), day_list_str).unwrap();
		}
	}
	fs::write("src/years/generated_list.rs", year_list).unwrap();
}
