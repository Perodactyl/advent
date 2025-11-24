use crate::lib::prelude::*;

#[derive(Debug, Clone, Copy, Display, EnumIs, PartialEq, Eq)]
enum Block {
	#[strum(to_string = ".")]
	Free(usize),
	#[strum(to_string = "{id}")]
	File { id: usize, size: usize },
}

///Prints a representation of a filesystem.
/// `<n>`: File of ID `n`
/// `.`: Free space
/// `_`: Free space directly after another free space
/// `|`: Zero-sized free space
fn print_fs(filesystem: &Vec<Block>) {
	if filesystem.len() > 30 { return };
	let mut last_was_free = false;
	for block in filesystem {
		match block {
			Block::Free(0) => {
				print!("|");
				last_was_free = false;
			},
			Block::Free(size) if last_was_free => {
				print!("{}", "_".repeat(*size));
				last_was_free = false;
			},
			Block::Free(size) => {
				print!("{}", ".".repeat(*size));
				last_was_free = true;
			},
			Block::File { id, size } => {
				print!("{}", format!("{id}").repeat(*size));
				last_was_free = false;
			},
		}
	}
	println!();
}

pub fn main(input: String) -> Result<String> {
	let mut filesystem = vec![];
	let mut file_count = 0;
	for (i,ch) in input.chars().enumerate() {
		let size = ch.to_string().parse::<usize>()?;
		if i % 2 == 0 {
			filesystem.push(Block::File { id: i / 2, size });
			file_count += 1;
		} else if size != 0 {
			filesystem.push(Block::Free(size));
		}
	}

	if !cfg!(feature = "part2") {
		let mut first_free_block = filesystem.iter().position(|b| b.is_free()).unwrap();

		while filesystem[filesystem.len()-1].is_free() {
			filesystem.pop();
		}

		print_fs(&filesystem);

		loop {
			if first_free_block == filesystem.len()-2 {
				filesystem.swap_remove(filesystem.len()-2);
				break;
			};

			let Block::Free(free_size) = filesystem[first_free_block] else { bail!("first_free_block is not free") };
			let Block::File { size: file_size, id } = filesystem[filesystem.len()-1] else { bail!("last_file_block is not a file: {filesystem:?}") };

			if free_size == file_size {
				filesystem.swap_remove(first_free_block);
			} else if free_size > file_size {
				filesystem.swap_remove(first_free_block);
				filesystem.insert(first_free_block+1, Block::Free(free_size - file_size));
			} else if file_size > free_size {
				filesystem[first_free_block] = Block::File { id, size: free_size };
				let last_file = filesystem.len() - 1;
				filesystem[last_file] = Block::File { id, size: file_size - free_size };
			}

			while filesystem[first_free_block].is_file() {
				first_free_block += 1;
			}
			while filesystem[filesystem.len()-1].is_free() {
				filesystem.pop();
			}
			if cfg!(feature = "sample") { print_fs(&filesystem) };
		}
	} else {
		// while filesystem[filesystem.len()-1].is_free() {
		// 	filesystem.pop();
		// }

		print_fs(&filesystem);

		for file_number in (0..=file_count).rev() {
			let Some((file_block, Block::File { size: file_size, .. })) = filesystem.iter()
				.enumerate()
				.find(|(_, b)| match b {
					Block::File { id, .. } => *id == file_number,
					_ => false
				}) else { continue };
			let file_size = *file_size;

			for i in 0..file_block {
				if match filesystem[i] {
					Block::Free(free_size) if free_size == file_size => {
						filesystem.swap(i, file_block);
						true
					},
					Block::Free(ref mut swapped_free_size) if *swapped_free_size > file_size => {
						let free_size = *swapped_free_size;
						*swapped_free_size = file_size;

						filesystem.swap(i, file_block);
						filesystem.insert(i+1, Block::Free(free_size - file_size));
						true
					},
					_ => false
				} {
					let mut i = 0; // Join free blocks
					while i < filesystem.len() {
						match (filesystem[i], filesystem.get(i+1).map(Clone::clone)) {
							(Block::Free(a), Some(Block::Free(b))) => {
								filesystem[i] = Block::Free(a+b);
								filesystem.remove(i+1);
								i -= 1;
							},
							_ => {}
						}
						i += 1;
					}
					if cfg!(feature = "sample") { print_fs(&filesystem) };
					break;
				}
			}
		}
	}

	print_fs(&filesystem);
	// println!("{filesystem:?}");

	let mut sum = 0;
	let mut offset = 0;
	for block in filesystem.iter() {
		match block {
			Block::File { id, size } => {
				for _ in 0..*size {
					sum += offset * id;
					offset += 1;
				}
			},
			Block::Free(size) => offset += size,
		}
	}

	Ok(format!("{sum}"))
}
