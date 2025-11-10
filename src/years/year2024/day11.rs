use std::collections::HashMap;

use crate::lib::prelude::*;

// Previously, I tried (and failed) to solve this by generating a tree on the heap. Then I tried it
// on the stack and it worked immediately instead of spending all 32GiB of RAM over the course of
// 16 seconds and then crashing trying to allocate more RAM. I'll use my second solution, then.

#[derive(Default)]
struct StoneCounter {
	memo: HashMap<(u64, usize), usize>,
} impl StoneCounter {
	fn stone_count_compute(&mut self, input: u64, iters: usize) -> usize {
		if iters == 0 { return 1 };
		if input == 0 { return self.stone_count(1, iters-1) };
		let stringified = input.to_string();
		if stringified.len() % 2 == 0 {
			let (left, right) = stringified.split_at(stringified.len() / 2);
			return self.stone_count(left.parse().unwrap(), iters-1) + self.stone_count(right.parse().unwrap(), iters-1);
		} else {
			return self.stone_count(input * 2024, iters-1);
		}
	}
	fn stone_count(&mut self, input: u64, iters: usize) -> usize {
		match self.memo.get(&(input, iters)) {
			None => {
				let output = self.stone_count_compute(input, iters);
				self.memo.insert((input, iters), output);
				output
			},
			Some(value) => *value
		}
	}
}


pub fn main(input: String) -> Result<String> {
	let mut total = 0;
	let mut counter = StoneCounter::default();
	for word in input.split_whitespace() {
		total += counter.stone_count(word.parse().unwrap(), if cfg!(feature = "part2") { 75 } else { 25 });
	}
	Ok(format!("{total}"))
}
