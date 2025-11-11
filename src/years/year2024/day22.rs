use crate::lib::prelude::*;

struct RNG(u32);
impl RNG {
	fn mix(&mut self, n: u32) {
		self.0 ^= n;
	}
	fn prune(&mut self) {
		self.0 &= 2u32.pow(24) - 1;
	}
	fn next(&mut self) {
		self.mix(self.0 << 6);
		self.prune();
		self.mix(self.0 >> 5);
		self.prune();
		self.mix(self.0 << 11);
		self.prune();
	}
}

pub fn main(input: String) -> Result<String> {
	let mut rngs = input.lines().map(|line| RNG(line.parse().unwrap())).collect::<Vec<_>>();
	for i in 0..2000 {
		// println!("\x1b[HIteration: {i}\x1b[K");
		for rng in &mut rngs {
			rng.next();
			// println!("{}\x1b[K", rng.0);
		}
		// println!("\x1b[J");
		// std::thread::sleep(std::time::Duration::from_millis(1));
	}
	Ok(format!("{}", rngs.into_iter().map(|rng| rng.0 as u64).sum::<u64>()))
}
