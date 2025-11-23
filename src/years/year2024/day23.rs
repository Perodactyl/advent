use crate::lib::prelude::*;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Ident([u8; 2]);
impl Debug for Ident {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	    write!(f, "{}{}", self.0[0] as char, self.0[1] as char)
	}
}
impl From<&str> for Ident {
	fn from(value: &str) -> Self {
	    assert!(value.len() == 2);
		let mut iter = value.bytes();
		Ident([
			iter.next().unwrap(),
			iter.next().unwrap()
		])
	}
}

#[derive(Clone)]
struct Computer {
	ident: Ident,
	connections: Vec<Ident>,
}

pub fn main(input: String) -> Result<String> {
	let mut computers = HashMap::new();
	for line in input.lines() {
		let (left, right) = line.split_once('-').unwrap();
		let left: Ident = left.try_into().unwrap();
		let right: Ident = right.try_into().unwrap();

		match computers.get_mut(&left) {
			None => {
				computers.insert(left, Computer {
					ident: left,
					connections: vec![right]
				});
			},
			Some(comp) => {
				comp.connections.push(right);
			}
		}

		match computers.get_mut(&right) {
			None => {
				computers.insert(right, Computer {
					ident: right,
					connections: vec![left],
				});
			},
			Some(comp) => {
				comp.connections.push(left);
			}
		}
	}

	let mut connected: Vec<(Ident, Ident, Ident)> = vec![];
	for (comp1_ident, comp1) in &computers {
		for comp2_ident in &comp1.connections {
			let comp2 = computers.get(comp2_ident).unwrap();
			for comp3_ident in &comp2.connections {
				if comp3_ident == comp1_ident { continue };
				let comp3 = computers.get(comp3_ident).unwrap();
				if comp3.connections.contains(comp1_ident) {
					let mut is_in_list = false;
					for (a, b, c) in &connected {
						if
							(a == comp1_ident && b == comp1_ident && c == comp1_ident) ||
							(a == comp2_ident && b == comp2_ident && c == comp2_ident) ||
							(a == comp3_ident && b == comp3_ident && c == comp3_ident) ||
							(a == comp1_ident && b == comp2_ident && c == comp3_ident) ||
							(a == comp2_ident && b == comp3_ident && c == comp1_ident) ||
							(a == comp3_ident && b == comp1_ident && c == comp2_ident) ||
							(a == comp3_ident && b == comp2_ident && c == comp1_ident) ||
							(a == comp2_ident && b == comp1_ident && c == comp3_ident)
						{
							is_in_list = true;
							break;
						}
					}
					if !is_in_list {
						connected.push((*comp1_ident, *comp2_ident, *comp3_ident));
					}
				}
			}
		}
	}
	println!("{:#?}", connected.len());
	Ok(format!(""))
}
