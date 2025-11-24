use std::cmp::Ordering;

use crate::lib::prelude::*;

pub fn test_safety(input: impl Iterator<Item = u8>) -> bool {
	let mut safe = true;
	let mut last_num: Option<u8> = None;
	let mut last_ord: Option<Ordering> = None;

	for num in input {
		if let Some(last_num) = last_num {
			let diff = last_num.abs_diff(num);
			if diff < 1 || diff > 3 {
				safe = false;
				print!("\x1b[31m{num}\x1b[39m");
				break;
			}

			if let Some(last_ord) = last_ord {
				if last_ord != num.cmp(&last_num) {
					safe = false;
					print!("\x1b[31m{num}\x1b[39m");
					break;
				}
			}
			last_ord = Some(num.cmp(&last_num));
		}

		print!("{num} ");
		last_num = Some(num);
	}

	println!();
	safe
}

pub fn main(input: String) -> Result<String> {
	let mut safe_reports = 0;

	'test_report: for line in input.lines() {
		let mut numbers: Vec<u8> = vec![];
		for num in line.split_whitespace() {
			numbers.push(num.parse()?);
		}

		if test_safety(numbers.iter().copied()) {
			safe_reports += 1;
			continue;
		} else if cfg!(feature = "part2") {
			for exclude in 0..numbers.len() {
				if test_safety(numbers.iter().enumerate().filter_map(|(i,n)| if i == exclude { None } else { Some(*n) })) {
					safe_reports += 1;
					continue 'test_report;
				}
			}
		}
	}

	Ok(format!("{safe_reports}"))
}
