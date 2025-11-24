use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
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
	pub fn right(&self) -> Direction {
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
	pub fn advance(&self, (x, y): (usize, usize)) -> Option<(usize, usize)> {
		use Direction::*;
		match self {
			Up    => Some(x).zip(y.checked_sub(1)),
			Down  => Some((x, y+1)),
			Left  => x.checked_sub(1).zip(Some(y)),
			Right => Some((x+1, y)),
		}
	}
}
