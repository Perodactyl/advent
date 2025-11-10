use crate::lib::prelude::*;

#[derive(Display, Debug, Clone, Copy, EnumIs, PartialEq, Eq)]
enum Cell {
	#[strum(to_string = "\x1b[2m.\x1b[22m")]
	None,
	#[strum(to_string = "\x1b[7m#\x1b[27m")]
	Wall,
	#[strum(to_string = "\x1b[31mO\x1b[39m")]
	Box,
	#[strum(to_string = "\x1b[1;33m@\x1b[22;39m")]
	Robot,
}

impl From<char> for Cell {
	fn from(value: char) -> Self {
	    match value {
			'.' => Cell::None,
			'#' => Cell::Wall,
			'O' => Cell::Box,
			'@' => Cell::Robot,
			ch => unimplemented!("{ch:?} cell")
		}
	}
}

fn push_box(x: usize, y: usize, ox: isize, oy: isize, grid: &mut ItemGrid<Cell>) -> bool {
	let dest_x = x.saturating_add_signed(ox);
	let dest_y = y.saturating_add_signed(oy);
	let should_push = match grid.get(dest_x, dest_y) {
		Cell::Wall => false,
		Cell::None => true,
		Cell::Box  => push_box(dest_x, dest_y, ox, oy, grid),
		Cell::Robot => unimplemented!("Robot push"),
	};
	if should_push {
		grid.set(dest_x, dest_y, Cell::Box);
		grid.set(x, y, Cell::None);
	}
	should_push
}

pub fn main(input: String) -> Result<String> {
	let (grid_text, moves_text) = input.split_once("\n\n").unwrap();
	let mut grid = ItemGrid::<Cell>::from(grid_text.trim());
	let (_, mut x, mut y) = grid.find(|c,_,_| c.is_robot()).unwrap();

	for ch in moves_text.trim().chars() {
		let (ox, oy) = match ch {
			'^' => ( 0, -1),
			'<' => (-1,  0),
			'>' => ( 1,  0),
			'v' => ( 0,  1),
			'\n' => continue,
			ch => unimplemented!("{ch:?} move"),
		};
		if push_box(x, y, ox, oy, &mut grid) {
			grid.set(x, y, Cell::None);
			x = x.saturating_add_signed(ox);
			y = y.saturating_add_signed(oy);
			grid.set(x, y, Cell::Robot);
		}
		// println!("Move {ch}:\n{grid}");
	}

	let mut sum = 0;
	for (x, y, tile) in grid.iter() {
		if *tile != Cell::Box { continue };
		sum += (y * 100) + x;
	}

	Ok(format!("{sum}"))
}
