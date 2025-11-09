use std::fmt::Display;

pub trait Grid<T> {
	fn get(&self, x: usize, y: usize) -> &T;
	fn get_mut(&mut self, x: usize, y: usize) -> &mut T;
	fn set(&mut self, x: usize, y: usize, value: T);
	fn width(&self) -> usize;
	fn height(&self) -> usize;

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
}

pub struct ConstSizeGrid<const W: usize, const H: usize, T> {
	items: [[T; W]; H],
} impl<const W: usize, const H: usize, T> ConstSizeGrid<W, H, T> {
	pub fn new_with(items: [[T; W]; H]) -> ConstSizeGrid<W, H, T> {
		ConstSizeGrid {
			items
		}
	}
	pub fn populated_with(populator: fn(x: usize, y: usize) -> T) -> ConstSizeGrid<W, H, T> {
		let mut items: Vec<[T; W]> = Vec::with_capacity(W);
		for x in 0..W {
			let mut column = Vec::with_capacity(H);
			for y in 0..H {
				column.push(populator(x, y));
			}
			items.push(unsafe { column.try_into().unwrap_unchecked() });
		}
		ConstSizeGrid::new_with(unsafe { items.try_into().unwrap_unchecked() })
	}
}

impl<const W: usize, const H: usize, T: Clone + Copy> ConstSizeGrid<W, H, T> {
	pub fn filled_with(default: T) -> ConstSizeGrid<W, H, T> {
		ConstSizeGrid::new_with([[default; W]; H])
	}
}

impl<const W: usize, const H: usize, T: Clone + Copy + Default> Default for ConstSizeGrid<W, H, T> {
	fn default() -> ConstSizeGrid<W, H, T> {
		ConstSizeGrid::filled_with(T::default())
	}
}

impl<const W: usize, const H: usize, T> Grid<T> for ConstSizeGrid<W, H, T> {
	fn get(&self, x: usize, y: usize) -> &T {
	    &self.items[x][y]
	}
	fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
	    &mut self.items[x][y]
	}
	fn set(&mut self, x: usize, y: usize, value: T) {
	    self.items[x][y] = value;
	}
	fn width(&self) -> usize {
	    W
	}
	fn height(&self) -> usize {
	    H
	}
}

pub struct ItemGrid<T> {
	items: Vec<Vec<T>>,
	width: usize,
	height: usize
}

impl<T> ItemGrid<T> {
	pub fn new() -> ItemGrid<T> {
		ItemGrid { items: vec![], width: 0, height: 0 }
	}
	pub fn add_col(&mut self, col: Vec<T>) {
		if self.width == 0 {
			self.height = col.len();
		} else {
			assert_eq!(self.height, col.len(), "Expected column of length {}, but got length {}", self.height, col.len());
		}

		self.width += 1;
		self.items.push(col);
	}
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
}

impl<T> Grid<T> for ItemGrid<T> {
	fn get(&self, x: usize, y: usize) -> &T {
	    &self.items[x][y]
	}
	fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
	    &mut self.items[x][y]
	}
	fn set(&mut self, x: usize, y: usize, value: T) {
	    self.items[x][y] = value;
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
