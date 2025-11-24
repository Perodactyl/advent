use crate::lib::prelude::*;

pub fn main(input: String) -> Result<String> {
	let mut left_list: Vec<u32> = vec![];
	let mut right_list: Vec<u32> = vec![];
	for line in input.lines() {
		let mut parts = line.split_whitespace();
		let left = parts.next().unwrap();
		let right = parts.next().unwrap();

		left_list.push(left.parse()?);
		right_list.push(right.parse()?);
	}

	if !cfg!(feature = "part2") {

		left_list.sort();
		right_list.sort();
		let mut sum = 0;

		for (left, right) in left_list.iter().zip(right_list.iter()) {
			sum += left.abs_diff(*right);
		}
		Ok(format!("{sum}"))

	} else {

		let mut sum = 0;
		for num in &left_list {
			for num2 in &right_list {
				if num2 == num {
					sum += num;
				}
			}
		}
		Ok(format!("{sum}"))

	}
}
