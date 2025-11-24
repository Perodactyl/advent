use crate::lib::prelude::*;

struct Parser {
	input: String,
	recall_stack: Vec<usize>,
	i: usize,
} impl Parser {
	fn next(&mut self) -> Option<char> {
		let output = self.input.chars().nth(self.i);
		self.i += 1;
		output
	}
	fn save(&mut self) {
		self.recall_stack.push(self.i);
	}
	fn recall(&mut self) {
		self.i = self.recall_stack.pop().unwrap();
	}
	fn discard(&mut self) {
		self.recall_stack.pop();
	}
	fn peek(&mut self) -> Option<char> {
		self.input.chars().nth(self.i)
	}
	///Returns true if a match was found and advances past the match. Otherwise, does not advance
	///at all.
	fn try_match(&mut self, target: &str) -> bool {
		self.save();
		for ch in target.chars() {
			if self.next() != Some(ch) {
				self.recall();
				return false;
			}
		}
		self.discard();
		true
	}
}

pub fn main(input: String) -> Result<String> {
	let mut enabled = true;
	let mut sum = 0;
	let mut parser = Parser { input, recall_stack: vec![], i: 0 };

	'try_parse_fn: while parser.i < parser.input.len() {
		parser.save();
		if parser.try_match("mul(") {
			let mut following = String::new();
			while parser.peek() != Some(')') {
				let Some(next) = parser.next() else { break 'try_parse_fn };
				following.push(next);
			}
			let Some((left, right)) = following.split_once(',') else { parser.recall(); parser.next(); continue };
			let Ok(left) = left.parse::<u32>() else { parser.recall(); parser.next(); continue };
			let Ok(right) = right.parse::<u32>() else { parser.recall(); parser.next(); continue };
			if enabled { sum += left * right };
			parser.discard();
		} else if cfg!(feature = "part2") && parser.try_match("do()") {
			enabled = true;
			parser.discard();
			continue;
		} else if cfg!(feature = "part2") && parser.try_match("don't()") {
			enabled = false;
			parser.discard();
			continue;
		}
		parser.next();
	}
	Ok(format!("{sum}"))
}
