#![allow(dead_code)]
use std::{collections::VecDeque, rc::Rc};

use crate::lib::grid::ItemGrid;

pub enum MapCell {
	Clear,
	Obstacle,
}

pub fn manhattan_distance(p1: (usize, usize), p2: (usize, usize)) -> f64 {
	(p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)) as f64
}

pub fn euclidiean_distance(p1: (usize, usize), p2: (usize, usize)) -> f64 {
	f64::sqrt((p1.0.abs_diff(p2.0).pow(2) + p1.1.abs_diff(p2.1).pow(2)) as f64)
}

struct Node {
	x: usize,
	y: usize,
	///Distance to start
	g: f64,
	///Distance to end
	h: f64,
	///Total cost
	f: f64,
	prev: Option<Rc<Node>>
}

pub struct AStarSearch {
	open: VecDeque<Rc<Node>>,
	closed: VecDeque<Rc<Node>>,
	map: ItemGrid<MapCell>,
	start: (usize, usize),
	end: (usize, usize),
	heuristic: fn((usize, usize), (usize, usize)) -> f64,
}
impl AStarSearch {
	pub fn new(
		map: ItemGrid<MapCell>,
		start: (usize, usize),
		end: (usize, usize),
		heuristic: fn((usize, usize), (usize, usize)) -> f64
	) -> AStarSearch {
		let mut output = AStarSearch {
			open: VecDeque::new(),
			closed: VecDeque::new(),
			map,
			start,
			end,
			heuristic
		};
		let h = heuristic(start, end);
		output.open.push_back(Rc::new(Node {
			x: start.0,
			y: start.1,
			g: 0.0,
			h,
			f: h,
			prev: None,
		}));

		output
	}
	pub fn step(&mut self) {
		let Some(node) = self.open.pop_front() else { return };
		self.closed.insert(self.closed.partition_point(|n| n.f < node.f), node);

	}
}
