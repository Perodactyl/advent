pub trait Grid<T> {
	fn get(&self, x: usize, y: usize) -> &T;
	fn get_mut(&mut self, x: usize, y: usize) -> &mut T;
	fn set(&mut self, x: usize, y: usize, value: T);
	fn width(&self) -> usize;
	fn height(&self) -> usize;
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
