use std::fmt::Display;

use crate::lib::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, EnumIs)]
enum Cell { //HashSet might be excessive (could just use 4 bools)
	Clear(HashSet<Direction>),
	Obstacle,
	GuardStart(Direction, HashSet<Direction>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
	Turned,
	Moved,
	OutOfBounds,
	Looping,
}

fn simulate_step(guard_dir: &mut Direction, guard_pos: &mut (usize, usize), grid: &mut ItemGrid<Cell>) -> State {
	match guard_dir.advance(*guard_pos) {
		None => State::OutOfBounds,
		Some((x, y)) if x >= grid.width() || y >= grid.height() => State::OutOfBounds,
		Some(new_pos @ (x, y)) => match grid.get_mut(x, y) {
			Cell::Obstacle => {
				*guard_dir = guard_dir.right();
				State::Turned
			},
			Cell::Clear(d) => {
				if d.contains(guard_dir) { return State::Looping };
				d.insert(*guard_dir);
				*guard_pos = new_pos;
				State::Moved
			},
			Cell::GuardStart(_, d) => {
				if d.contains(guard_dir) { return State::Looping };
				d.insert(*guard_dir);
				*guard_pos = new_pos;
				State::Moved
			}
		}
	}
}

pub fn main(input: String) -> Result<String> {
	let mut grid = ItemGrid::<Cell>::new();
	for line in input.lines() {
		let mut row = vec![];
		for ch in line.chars() {
			match ch {
				'.' => row.push(Cell::Clear(HashSet::new())),
				'#' => row.push(Cell::Obstacle),
				'^' => row.push(Cell::GuardStart(Direction::Up, HashSet::new())),
				'v' => row.push(Cell::GuardStart(Direction::Down, HashSet::new())),
				'<' => row.push(Cell::GuardStart(Direction::Left, HashSet::new())),
				'>' => row.push(Cell::GuardStart(Direction::Right, HashSet::new())),
				c => panic!("Unexpected character {c} in input map")
			}
		}
		grid.add_row(row);
	}
	
	let Some((Cell::GuardStart(guard_dir, _), guard_x, guard_y)) = grid.find(|cell, _, _| {
		matches!(cell, Cell::GuardStart(_, _))
	}) else { panic!("Guard not found") };

	let original_guard_dir = guard_dir.clone();
	let original_guard_pos = (guard_x, guard_y);
	let original_grid = grid;

	if !cfg!(feature = "part2") {
		let mut grid = original_grid;
		let mut guard_dir = original_guard_dir;
		let mut guard_pos = original_guard_pos;
		loop {
			match simulate_step(&mut guard_dir, &mut guard_pos, &mut grid) {
				State::OutOfBounds => break,
				_ => {}
			}
		}
		let mut sum = 0;
		for (_, _, cell) in grid.iter() {
			match cell {
				Cell::Clear(d) => if !d.is_empty() { sum += 1 },
				Cell::GuardStart(_, d) => if !d.is_empty() { sum += 1 },
				Cell::Obstacle => {}
			}
		}
		Ok(format!("{sum}"))
	} else {
		let regular_path = {
			let mut grid = original_grid.clone();
			let mut guard_dir = original_guard_dir;
			let mut guard_pos = original_guard_pos;
			loop {
				match simulate_step(&mut guard_dir, &mut guard_pos, &mut grid) {
					State::OutOfBounds => break,
					_ => {}
				}
			}

			grid
		};

		let mut sum = 0;
		'find_loops: for (_, x, y) in regular_path.find_each(|c,_,_| match c {
			Cell::Clear(d) => !d.is_empty(),
			Cell::GuardStart(_, d) => !d.is_empty(),
			Cell::Obstacle => false
		}) {
			let mut grid = original_grid.clone();
			let mut guard_dir = original_guard_dir;
			let mut guard_pos = original_guard_pos;

			grid.set(x, y, Cell::Obstacle);

			loop {
				match simulate_step(&mut guard_dir, &mut guard_pos, &mut grid) {
					State::Looping => {
						sum += 1;
						println!("{sum}");
						continue 'find_loops;
					},
					State::OutOfBounds => break,
					_ => {}
				}
			}
		}

		Ok(format!("{sum}"))
	}
}
