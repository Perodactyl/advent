//! Structures for storing and retrieving data in 2D grids

use std::fmt::Display;

///A neighborhood of 8 adjacent members, excluding the middle cell
pub const MOORE_NEIGHBORHOOD: &'static [(isize, isize); 8] = &[
	(-1, -1), ( 0, -1), ( 1, -1),
	(-1,  0), /*Self,*/ ( 1,  0),
	(-1,  1), ( 0,  1), ( 1,  1),
];

///A neighborhood of 4 adjacent members, excluding the middle cell and diagonals
pub const VON_NEUMANN_NEIGHBORHOOD: &'static [(isize, isize); 4] = &[
	/*None,*/ ( 0, -1), /*None,*/
	(-1,  0), /*Self,*/ ( 1,  0),
	/*None,*/ ( 0,  1), /*None,*/
];

///Stores elements in a 2D grid.
pub trait Grid<T> {
	///Gets an element at a specific position, or None if it is out of bounds.
	fn get_checked(&self, x: usize, y: usize) -> Option<&T>;
	///Gets an element at a specific position, or None if it is out of bounds.
	fn get_mut_checked(&mut self, x: usize, y: usize) -> Option<&mut T>;
	///Sets an element at a specific position. Returns None if it is out of bounds, or the element
	///that was previously there.
	fn set_checked(&mut self, x: usize, y: usize, value: T) -> Option<T>;
	///Gets an element at a specific position. Panics if it is out of bounds.
	fn get(&self, x: usize, y: usize) -> &T {
		self.get_checked(x, y).unwrap()
	}
	///Gets an element at a specific position. Panics if it is out of bounds.
	fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
		self.get_mut_checked(x, y).unwrap()
	}
	///Sets an element at a specific position. Panics if it is out of boundss.
	fn set(&mut self, x: usize, y: usize, value: T) -> T {
		self.set_checked(x, y, value).unwrap()
	}
	///Gets the current width of grid.
	fn width(&self) -> usize;
	///Gets the current height of grid.
	fn height(&self) -> usize;

	///Returns the first element and its coordinates on which `predicate` returns true, or None if none was
	///found. This function is short-circuiting.
	fn find(&self, predicate: fn(element: &T, x: usize, y: usize) -> bool) -> Option<(&T, usize, usize)> {
		for x in 0..self.width() {
			for y in 0..self.height() {
				let cell = self.get(x, y);
				if predicate(cell, x, y) {
					return Some((cell, x, y));
				}
			}
		}

		None
	}
	///Returns a Neighborhood around a certain cell.
	fn get_neighborhood(&self, x: usize, y: usize, structure: &[(isize, isize)]) -> Neighborhood {
		let mut results = Vec::with_capacity(8);
		for (ox, oy) in structure.iter().copied() {
			let Some(neighbor_x) = x.checked_add_signed(ox) else { continue };
			let Some(neighbor_y) = y.checked_add_signed(oy) else { continue };
			let Some(_) = self.get_checked(neighbor_x, neighbor_y) else { continue };
			results.push(NeighborhoodMember {
				rel_x: ox,
				rel_y: oy,
				abs_x: neighbor_x,
				abs_y: neighbor_y,
			});
		}

		results.shrink_to_fit();

		results
	}
}

///Stores the absolute location of a cell as well as its location relative to the center of a
///neighborhood.
pub struct NeighborhoodMember {
	pub rel_x: isize,
	pub rel_y: isize,
	pub abs_x: usize,
	pub abs_y: usize,
}

///Stores members of a neighborhood. Does not store info about what type of neighborhood it is.
pub type Neighborhood = Vec<NeighborhoodMember>;

///Grid structure for storing a list of items whose size is known at compile time.
pub struct ConstSizeGrid<const W: usize, const H: usize, T> {
	items: [[T; W]; H],
} impl<const W: usize, const H: usize, T> ConstSizeGrid<W, H, T> {
	///Creates a grid with the specified list of columns
	pub fn new_with_cols(items: [[T; W]; H]) -> ConstSizeGrid<W, H, T> {
		ConstSizeGrid {
			items
		}
	}
	///Creates a grid with the specified list of rows
	pub fn new_with_rows(items: [[T; H]; W]) -> ConstSizeGrid<W, H, T> {
		let _ = items;
		unimplemented!()
	}

	///Creates a grid using a function to generate each element.
	pub fn populated_with(populator: fn(x: usize, y: usize) -> T) -> ConstSizeGrid<W, H, T> {
		let mut items: Vec<[T; W]> = Vec::with_capacity(W);
		for x in 0..W {
			let mut column = Vec::with_capacity(H);
			for y in 0..H {
				column.push(populator(x, y));
			}
			items.push(unsafe { column.try_into().unwrap_unchecked() });
		}
		ConstSizeGrid::new_with_cols(unsafe { items.try_into().unwrap_unchecked() })
	}
}

impl<const W: usize, const H: usize, T: Clone + Copy> ConstSizeGrid<W, H, T> {
	///Creates a grid using a copied value.
	pub fn filled_with(default: T) -> ConstSizeGrid<W, H, T> {
		ConstSizeGrid::new_with_cols([[default; W]; H])
	}
}

impl<const W: usize, const H: usize, T: Clone + Copy + Default> Default for ConstSizeGrid<W, H, T> {
	///Creates a grid using a copied default value.
	fn default() -> ConstSizeGrid<W, H, T> {
		ConstSizeGrid::filled_with(T::default())
	}
}

impl<const W: usize, const H: usize, T> Grid<T> for ConstSizeGrid<W, H, T> {
	fn get_checked(&self, x: usize, y: usize) -> Option<&T> {
	    self.items.get(x)?.get(y)
	}
	fn get_mut_checked(&mut self, x: usize, y: usize) -> Option<&mut T> {
		self.items.get_mut(x)?.get_mut(y)
	}
	fn set_checked(&mut self, x: usize, y: usize, value: T) -> Option<T> {
		if x >= self.width() || y >= self.height() { return None };
	    Some(std::mem::replace(&mut self.items[x][y], value))
	}
	fn width(&self) -> usize {
	    W
	}
	fn height(&self) -> usize {
	    H
	}
}

///Grid structure for storing a list of items whose size is unknown at compile time.
pub struct ItemGrid<T> {
	items: Vec<Vec<T>>,
	width: usize,
	height: usize
}

impl<T> ItemGrid<T> {
	///Creates an empty ItemGrid
	pub fn new() -> ItemGrid<T> {
		ItemGrid { items: vec![], width: 0, height: 0 }
	}
	///Adds a column to the grid. If the grid is empty, the height is set by the column. Otherwise,
	///panics if the colum length does not match.
	pub fn add_col(&mut self, col: Vec<T>) {
		if self.width == 0 {
			self.height = col.len();
		} else {
			assert_eq!(self.height, col.len(), "Expected column of length {}, but got length {}", self.height, col.len());
		}

		self.width += 1;
		self.items.push(col);
	}

	///Adds a row to the grid. If the grid is empty, the width is set by the row. Otherwise, panics
	///if the row length does not match.
	pub fn add_row(&mut self, row: Vec<T>) {
		if self.height == 0 {
			self.width = row.len();
			for _ in 0..self.width {
				self.items.push(vec![]);
			}
		} else {
			assert_eq!(self.width, row.len(), "Expected row of length {}, but got length {}", self.width, row.len());
		}

		self.height += 1;
		for (col, val) in self.items.iter_mut().zip(row) {
			col.push(val);
		}
	}

	///Shrinks the capacity of each column and the capacity for more columns as much as possible.
	pub fn shrink_to_fit(&mut self) {
		for col in &mut self.items {
			col.shrink_to_fit();
		}
		self.items.shrink_to_fit();
	}
}

impl<T> Grid<T> for ItemGrid<T> {
	fn get_checked(&self, x: usize, y: usize) -> Option<&T> {
	    self.items.get(x)?.get(y)
	}
	fn get_mut_checked(&mut self, x: usize, y: usize) -> Option<&mut T> {
		self.items.get_mut(x)?.get_mut(y)
	}
	fn set_checked(&mut self, x: usize, y: usize, value: T) -> Option<T> {
		if x >= self.width() || y >= self.height() { return None };
	    Some(std::mem::replace(&mut self.items[x][y], value))
	}
	fn width(&self) -> usize {
	    self.width
	}
	fn height(&self) -> usize {
	    self.height
	}
}

impl<T: Display> Display for ItemGrid<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for y in 0..self.height {
			for x in 0..self.width {
				write!(f, "{}", self.get(x, y))?;
			}
			writeln!(f)?;
		}
		Ok(())
	}
}

impl<T: From<char>> From<&str> for ItemGrid<T> {
	fn from(value: &str) -> Self {
	    let mut output = ItemGrid::new();
		for line in value.lines() {
			output.add_row(line.chars().map(Into::into).collect());
		}

		output.shrink_to_fit();
		output
	}
}
