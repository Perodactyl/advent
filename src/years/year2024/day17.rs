use crate::lib::prelude::*;

#[derive(Default, Debug, Clone)]
struct CPU {
	a: i64,
	b: i64,
	c: i64,
	pc: usize,
	program: Vec<u8>,
	output: Vec<i64>,
} impl CPU {
	fn tick(&mut self) {
		let opcode = self.program[self.pc];
		let literal_operand = self.program[self.pc+1];
		let combo_operand = match literal_operand {
			4 => self.a,
			5 => self.b,
			6 => self.c,
			val => val as i64,
		};
		let mut next_pc = self.pc + 2;
		let opcode_str;
		let source_is_combo;
		let old_a = self.a;
		let old_b = self.b;
		let old_c = self.c;

		match opcode {
			/* adv */ 0 => { //Divide A by 2**N and store in A
				opcode_str = "adv";
				source_is_combo = true;
				self.a = self.a >> combo_operand
			},
			/* bxl */ 1 => { //XOR B by L and store in B
				opcode_str = "bxl";
				source_is_combo = false;
				self.b = self.b ^ literal_operand as i64;
			}
			/* bst */ 2 => { //Modulo N by 8 and store in B
				opcode_str = "bst";
				source_is_combo = true;
				self.b = combo_operand & 0b111;
			},
			/* jnz */ 3 => { //Jump to L if A != 0
				opcode_str = "jnz";
				source_is_combo = false;
				if self.a != 0 { next_pc = literal_operand as usize };
			},
			/* bxc */ 4 => { //XOR B by C and store in B
				opcode_str = "bxc";
				source_is_combo = true;
				self.b = self.b ^ self.c;
			},
			/* out */ 5 => { //Append N mod 8 to output
				opcode_str = "out";
				source_is_combo = true;
				self.output.push(combo_operand & 0b111);
			},
			/* bdv */ 6 => { //adv but stored in B
				opcode_str = "bdv";
				source_is_combo = true;
				self.b = self.a >> combo_operand;
			},
			/* cdv */ 7 => { //adv but stored in C
				opcode_str = "cdv";
				source_is_combo = true;
				self.c = self.a >> combo_operand;
			}
			c => unimplemented!("opcode {c:?}")
		}

		println!(
			"pc={:<2} a={old_a:<3} b={old_b:<3} c={old_c:<3} {opcode_str} {}",
			self.pc, if source_is_combo {
				match literal_operand {
					4 => String::from("A"),
					5 => String::from("B"),
					6 => String::from("C"),
					n => n.to_string()
				}
			} else { format!("#{literal_operand}") }
		);

		self.pc = next_pc;
	}
}

pub fn main(input: String) -> Result<String> {
	let mut cpu = CPU::default();
	for line in input.lines() {
		if line.starts_with("Register A: ") {
			cpu.a = line["Register A: ".len()..].parse().unwrap();
		}
		if line.starts_with("Register B: ") {
			cpu.b = line["Register B: ".len()..].parse().unwrap();
		}
		if line.starts_with("Register C: ") {
			cpu.c = line["Register C: ".len()..].parse().unwrap();
		}
		if line.starts_with("Program: ") {
			cpu.program = line["Program: ".len()..].split(",").map(|v| v.parse().unwrap()).collect();
		}
	}
	println!("{cpu:?}");
	while cpu.pc < cpu.program.len() {
		cpu.tick();
	}
	println!("{cpu:?}");
	Ok(cpu.output.iter().map(i64::to_string).collect::<Vec<_>>().join(","))
}
