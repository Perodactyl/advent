use crate::lib::prelude::*;

#[derive(Debug, Clone, Copy, Default, Display, EnumIs, PartialEq, Eq)]
enum Cell {
	#[default]
	#[strum(to_string = ".")]
	Clear,
	#[strum(to_string = ".")]
	Filled,
}
impl From<char> for Cell {
	fn from(value: char) -> Self {
		match value {
			'.' => Cell::Clear,
			'#' | _ => Cell::Filled
		}
	}
}

#[derive(Debug)]
struct Lock(Vec<usize>);
#[derive(Debug)]
struct Key(Vec<usize>);

pub fn main(input: String) -> Result<String> {
	let mut locks = vec![];
	let mut keys = vec![];
	let mut max_height = None;

	for part in input.split("\n\n") {
		let grid = ItemGrid::<Cell>::try_from(part)?;
		if let Some(max_height) = max_height {
			if grid.height() != max_height { bail!("Invalid part") };
		} else {
			max_height = Some(grid.height());
		}
		
		let mut is_lock = true;
		for cell in grid.row(0) {
			if *cell == Cell::Clear {
				is_lock = false;
				break;
			}
		}

		let mut heights = Vec::with_capacity(grid.width());
		for x in 0..grid.width() {
			let filled = grid.col(x).iter().filter(|c| c.is_filled()).count();
			heights.push(filled-1);
		}
		if is_lock {
			locks.push(Lock(heights));
		} else {
			keys.push(Key(heights));
		}
	}

	let Some(max_height) = max_height else { bail!("No parts checked") };
	let max_height = max_height - 2;
	println!("Max height: {max_height}");
	let mut matches = 0;

	init_progress_bar_with_eta(keys.len() * locks.len());

	for key in keys.iter() {
		for lock in locks.iter() {
			let mut valid = true;
			for (k, l) in key.0.iter().zip(lock.0.iter()) {
				if k + l > max_height {
					valid = false;
					break;
				}
			}
			if valid {
				print_progress_bar_info("Valid", &format!("{key:?} vs. {lock:?}"), Color::Green, Style::Bold);
			} else {
				print_progress_bar_info("Invalid", &format!("{key:?} vs. {lock:?}"), Color::Red, Style::Bold);
			}
			inc_progress_bar();
			if valid {
				matches += 1;
			}
		}
	}

	finalize_progress_bar();
	println!("{} keys vs {} locks", keys.len(), locks.len());

	Ok(format!("{matches}"))
}
