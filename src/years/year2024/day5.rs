use crate::lib::prelude::*;

pub fn main(input: String) -> Result<String> {
	//Each rule says "X precedes Y"
	let mut rules = Vec::<(u32, u32)>::new();
	let mut updates = Vec::<Vec<u32>>::new();

	let mut is_parsing_updates = false;
	for line in input.lines() {
		if line.is_empty() { is_parsing_updates = true; continue };
		if !is_parsing_updates {
			let Some((left, right)) = line.split_once('|') else { bail!("Not a rule: {line}") };
			let left = left.parse()?;
			let right = right.parse()?;
			rules.push((left, right));
		} else {
			let mut update = vec![];
			for num in line.split(',') {
				update.push(num.parse()?);
			}
			updates.push(update);
		}
	}

	let mut sum = 0;
	for mut update in updates {
		let mut valid = true;
		'reorder_loop: loop { // < loop breaks if not explicitly continued (by part2)
			'check_pages: for i in 0..update.len() {
				for rule in &rules {
					if rule.1 == update[i] && let Some(j) = update.iter().position(|n| *n == rule.0) {
						if i < j {
							if cfg!(feature = "part2") {
								valid = false;
								println!("{}|{} broken", rule.0, rule.1);
								let el = update.remove(j); // Move J so it precedes I
								update.insert(i, el);
								continue 'reorder_loop;
							} else {
								valid = false;
								println!("{}|{} broken", rule.0, rule.1);
								break 'check_pages;
							}
						}
					}
				}
			}
			break;
		}

		if valid ^ cfg!(feature = "part2") { // Valid updates in part 1; invalid ones in part 2
			sum += update[update.len() / 2];
		}
	}
	Ok(format!("{sum}"))
}
