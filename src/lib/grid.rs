use std::fmt::Display;

pub const MOORE_NEIGHBORHOOD: &'static [(isize, isize); 8] = &[
	(-1, -1), ( 0, -1), ( 1, -1),
	(-1,  0), /*Self,*/ ( 1,  0),
	(-1,  1), ( 0,  1), ( 1,  1),
];

pub const VON_NEUMANN_NEIGHBORHOOD: &'static [(isize, isize); 4] = &[
	/*None,*/ ( 0, -1), /*None,*/
	(-1,  0), /*Self,*/ ( 1,  0),
	/*None,*/ ( 0,  1), /*None,*/
];

pub trait Grid<T> {
	fn get_checked(&self, x: usize, y: usize) -> Option<&T>;
	fn get_mut_checked(&mut self, x: usize, y: usize) -> Option<&mut T>;
	fn set_checked(&mut self, x: usize, y: usize, value: T) -> Option<T>;
	fn get(&self, x: usize, y: usize) -> &T {
		self.get_checked(x, y).unwrap()
	}
	fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
		self.get_mut_checked(x, y).unwrap()
	}
	fn set(&mut self, x: usize, y: usize, value: T) -> T {
		self.set_checked(x, y, value).unwrap()
	}
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
	fn get_neighborhood(&self, x: usize, y: usize, structure: &[(isize, isize)]) -> Neighborhood<&T> {
		let mut results = Vec::with_capacity(8);
		for (ox, oy) in structure.iter().copied() {
			let Some(neighbor_x) = x.checked_add_signed(ox) else { continue };
			let Some(neighbor_y) = y.checked_add_signed(oy) else { continue };
			let Some(cell) = self.get_checked(neighbor_x, neighbor_y) else { continue };
			results.push(NeighboorhoodMember {
				rel_x: ox,
				rel_y: oy,
				abs_x: neighbor_x,
				abs_y: neighbor_y,
				item: cell
			});
		}

		results.shrink_to_fit();

		Neighborhood { members: results }
	}
}

pub struct NeighboorhoodMember<T> {
	pub rel_x: isize,
	pub rel_y: isize,
	pub abs_x: usize,
	pub abs_y: usize,
	pub item: T,
} impl<T> NeighboorhoodMember<T> {
	fn unit(self) -> NeighboorhoodMember<()> {
		let NeighboorhoodMember { rel_x, rel_y, abs_x, abs_y, item: _ } = self;
		NeighboorhoodMember {
			rel_x,
			rel_y,
			abs_x,
			abs_y,
			item: (),
		}
	}
}

pub struct Neighborhood<T> {
	members: Vec<NeighboorhoodMember<T>>
} impl<T> Neighborhood<T> {
	pub fn get(&self, offset_x: isize, offset_y: isize) -> Option<&T> {
		Some(&self.members.iter().find(|m| m.rel_x == offset_x && m.rel_y == offset_y)?.item)
	}
	pub fn get_mut(&mut self, offset_x: isize, offset_y: isize) -> Option<&mut T> {
		let position = self.members.iter().position(|m| m.rel_x == offset_x && m.rel_y == offset_y)?;
		Some(&mut self.members[position].item)
	}
	///Removes items from members so the original is no longer borrowed
	pub fn of_units(self) -> Neighborhood<()> {
		Neighborhood {
			members: self.members.into_iter().map(NeighboorhoodMember::unit).collect()
		}
	}
} impl<T> IntoIterator for Neighborhood<T> {
	type Item = NeighboorhoodMember<T>;
	type IntoIter = std::vec::IntoIter<Self::Item>;
	fn into_iter(self) -> Self::IntoIter {
	    self.members.into_iter()
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
