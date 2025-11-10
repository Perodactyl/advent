use std::fmt::Display;

use crate::lib::prelude::*;

#[derive(Debug, Clone, Copy)]
struct Cell {
	ch: char,
	visited: bool,
} impl From<char> for Cell {
	fn from(ch: char) -> Cell {
		Cell {
			ch,
			visited: false,
		}
	}
} impl Display for Cell {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	    match self.visited {
			true => write!(f, "\x1b[32;7m{}\x1b[39;27m", self.ch.to_ascii_lowercase()),
			false => write!(f, "{}", self.ch),
		}
	}
}

fn compute_area_and_perim((ch, x, y): (char, usize, usize), grid: &mut impl Grid<Cell>) -> (u32, u32) {
	let mut area = 1;
	let mut perim = 0;
	grid.get_mut(x, y).visited = true;

	for
		NeighborhoodMember { abs_x, abs_y, abs_unbounded_x, abs_unbounded_y, .. }
		in grid.get_neighborhood(x, y, VON_NEUMANN_NEIGHBORHOOD, false)
	{
		if abs_unbounded_x >= 0 && abs_unbounded_y >= 0 && let Some(item) = grid.get_checked(abs_x, abs_y) && item.ch == ch {
			// perim -= 1;
			if item.visited { continue };

			let (recurse_area, recurse_perim) = compute_area_and_perim((ch, abs_x, abs_y), grid);
			area += recurse_area;
			perim += recurse_perim;
		} else {
			perim += 1;
		}
	}
	(area, perim)
}

pub fn main(input: String) -> Result<String> {
	let mut grid = ItemGrid::<Cell>::from(&input[..]);
	let mut sum = 0;
	loop {
		let Some((start_cell, start_x, start_y)) = grid.find(|c, _, _| c.visited == false) else { break };
		let ch = start_cell.ch;
		let (area, perimeter) = compute_area_and_perim((ch, start_x, start_y), &mut grid);
		println!("{ch}: {area} * {perimeter} = {}", area * perimeter);
		sum += area * perimeter;
	}

	Ok(format!("{sum}"))
}
