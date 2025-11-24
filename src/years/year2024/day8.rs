use crate::lib::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Directions {
	horizontal: Vec<usize>,
	vertical: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIs)]
enum Cell {
	Antenna(char, HashMap<char, Directions>),
	Clear(HashMap<char, Directions>)
} impl Display for Cell {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	    match self {
			Cell::Antenna(_, d) | Cell::Clear(d) => {
				if self.is_antenna() {
					write!(f, "\x1b[31m")?;
				}
				if d.is_empty() {
					write!(f, "\x1b[2m.\x1b[22m")?;
				} else {
					for (_, dirs) in d {
						if dirs.horizontal.is_empty() && dirs.vertical.is_empty() {
							write!(f, "\x1b[2m.\x1b[22m")?;
						} else if dirs.horizontal.is_empty() && !dirs.vertical.is_empty() {
							write!(f, "|")?;
						} else if !dirs.horizontal.is_empty() && dirs.vertical.is_empty() {
							write!(f, "-")?;
						} else {
							write!(f, "+")?;
						}
						break;
					}
				}

				if self.is_antenna() {
					write!(f, "\x1b[39m")?;
				}
				Ok(())
			}
		}
	}
}

impl From<char> for Cell {
	fn from(value: char) -> Self {
	    match value {
			'.' => Cell::Clear(HashMap::new()),
			c => Cell::Antenna(c, HashMap::new()),
		}
	}
}

pub fn main(input: String) -> Result<String> {
	let mut grid = ItemGrid::<Cell>::try_from(input.as_str())?;
	for x in 0..grid.width() {
		for y in 0..grid.height() {
			let Cell::Antenna(ch, _) = grid.get(x, y) else { continue };
			let ch = ch.clone();

			for other_x in 0..grid.width() {
				match grid.get_mut(other_x, y) {
					Cell::Clear(d) | Cell::Antenna(_, d) => {
						let dirs;
						if let Some(directions) = d.get_mut(&ch) {
							dirs = directions;
						} else {
							d.insert(ch, Directions::default());
							let Some(new_dirs) = d.get_mut(&ch) else { unreachable!() };
							dirs = new_dirs;
						}
						
						dirs.horizontal.push(x.abs_diff(other_x));
					}
				}
			}

			for other_y in 0..grid.height() {
				match grid.get_mut(x, other_y) {
					Cell::Clear(d) | Cell::Antenna(_, d) => {
						let dirs;
						if let Some(directions) = d.get_mut(&ch) {
							dirs = directions;
						} else {
							d.insert(ch, Directions::default());
							let Some(new_dirs) = d.get_mut(&ch) else { unreachable!() };
							dirs = new_dirs;
						}
						
						dirs.vertical.push(y.abs_diff(other_y));
					}
				}
			}
		}
	}

	let mut sum = 0;
	for x in 0..grid.width() {
		'check_cell: for y in 0..grid.height() {
			match grid.get(x, y) {
				Cell::Clear(d) | Cell::Antenna(_, d) => {
					for (_, group) in d {
						for h in &group.horizontal {
							for v in &group.vertical {
								if *h == v * 2 || *v == h * 2 {
									sum += 1;
									continue 'check_cell;
								}
							}
						}
					}
				}
			}
		}
	}
	println!("{grid}");

	Ok(format!("{sum}"))
}
