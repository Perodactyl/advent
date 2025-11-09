use std::fmt::Display;

use crate::lib::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
	Up,
	Down,
	Left,
	Right
} impl Display for Direction {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		use Direction::*;
	    if f.alternate() {
			match self {
				Up | Down => write!(f, "|"),
				Left | Right => write!(f, "-"),
			}
		} else {
			match self {
				Up => write!(f, "^"),
				Down => write!(f, "v"),
				Left => write!(f, "<"),
				Right => write!(f, ">"),
			}
		}
	}
}
impl Direction {
	fn right(&self) -> Direction {
		use Direction::*;
		match self {
			Up    => Right,
			Right => Down,
			Down  => Left,
			Left  => Up,
		}
	}
	///Moves the inputted point one unit in the direction of self. Returns None if a coordinate
	///would go negative.
	fn advance(&self, (x, y): (usize, usize)) -> Option<(usize, usize)> {
		use Direction::*;
		match self {
			Up    => Some(x).zip(y.checked_sub(1)),
			Down  => Some((x, y+1)),
			Left  => x.checked_sub(1).zip(Some(y)),
			Right => Some((x+1, y)),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumIs)]
enum Cell {
	#[strum(to_string = "\x1b[2;30m.\x1b[22;39m")]
	Clear,
	#[strum(to_string = "\x1b[1m#\x1b[22m")]
	Obstacle,
	#[strum(to_string = "\x1b[31m{0}\x1b[39m")]
	GuardStart(Direction),
	#[strum(to_string = "\x1b[34m{0}\x1b[39m")]
	Traversed(Direction),
}

pub fn main(input: String) -> Result<String> {
	let mut grid = ItemGrid::<Cell>::new();
	for line in input.lines() {
		let mut row = vec![];
		for ch in line.chars() {
			match ch {
				'.' => row.push(Cell::Clear),
				'#' => row.push(Cell::Obstacle),
				'^' => row.push(Cell::GuardStart(Direction::Up)),
				'v' => row.push(Cell::GuardStart(Direction::Down)),
				'<' => row.push(Cell::GuardStart(Direction::Left)),
				'>' => row.push(Cell::GuardStart(Direction::Right)),
				c => panic!("Unexpected character {c} in input map")
			}
		}
		grid.add_row(row);
	}
	
	let Some((Cell::GuardStart(guard_dir), guard_x, guard_y)) = grid.find(|cell, _, _| {
		matches!(cell, Cell::GuardStart(_))
	}) else { panic!("Guard not found") };
	let mut guard_dir = guard_dir.clone();
	let mut guard_pos = (guard_x, guard_y);

	loop {
		match guard_dir.advance(guard_pos) {
			None => break,
			Some((x, y)) if x >= grid.width() || y >= grid.height() => break,
			Some(new_pos @ (x, y)) => match grid.get(x, y) {
				Cell::GuardStart(s) if *s == guard_dir => break,
				Cell::Obstacle => {
					guard_dir = guard_dir.right();
				},
				Cell::Clear | Cell::Traversed(_) => {
					grid.set(guard_pos.0, guard_pos.1, Cell::Traversed(guard_dir));
					guard_pos = new_pos;
				},
				Cell::GuardStart(_) => {
					guard_pos = new_pos;
				}
			}
		}
		println!("{guard_dir} {guard_pos:?}");
	}

	println!("{grid}");
	Ok(String::new())
}
