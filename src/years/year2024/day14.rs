use crate::lib::prelude::*;

#[derive(Debug, Clone, Copy)]
struct Robot {
	position: (isize, isize),
	velocity: (isize, isize),
}

impl Display for Robot {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let Robot { position: (x, y), velocity: (dx, dy) } = self;
		write!(f, "p={x},{y} v={dx},{dy}")
	}
}

#[cfg(feature = "sample")]
const WIDTH: isize = 11;
#[cfg(feature = "sample")]
const HEIGHT: isize = 7;

#[cfg(not(feature = "sample"))]
const WIDTH: isize = 101;
#[cfg(not(feature = "sample"))]
const HEIGHT: isize = 103;

const WIDTH_U: usize = WIDTH as usize;
const HEIGHT_U: usize = HEIGHT as usize;

pub fn main(input: String) -> Result<String> {
	let mut robots = vec![];
	for line in input.lines() {
		let (pos_part, vel_part) = line.split_once(' ').unwrap();

		let pos_part = &pos_part[2..];
		let (pos_x, pos_y) = pos_part.split_once(',').unwrap();

		let vel_part = &vel_part[2..];
		let (vel_x, vel_y) = vel_part.split_once(',').unwrap();

		robots.push(Robot {
			position: (pos_x.parse().unwrap(), pos_y.parse().unwrap()),
			velocity: (vel_x.parse().unwrap(), vel_y.parse().unwrap()),
		});
	}
	if cfg!(not(feature = "part2")) {
		for _ in 0..100 {
			for robot in &mut robots {
				robot.position.0 += robot.velocity.0;
				robot.position.1 += robot.velocity.1;
				while robot.position.0 >= WIDTH  { robot.position.0 -= WIDTH  };
				while robot.position.0 < 0       { robot.position.0 += WIDTH  };
				while robot.position.1 >= HEIGHT { robot.position.1 -= HEIGHT };
				while robot.position.1 < 0       { robot.position.1 += HEIGHT };
			}
		}
		let mut quadrants = [0; 4];
		for robot in &robots {
			if robot.position.0 < WIDTH/2 && robot.position.1 < HEIGHT/2 {
				quadrants[0] += 1;
			}
			if robot.position.0 > WIDTH/2 && robot.position.1 < HEIGHT/2 {
				quadrants[1] += 1;
			}
			if robot.position.0 < WIDTH/2 && robot.position.1 > HEIGHT/2 {
				quadrants[2] += 1;
			}
			if robot.position.0 > WIDTH/2 && robot.position.1 > HEIGHT/2 {
				quadrants[3] += 1;
			}
		}
		Ok(format!("{}", quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3]))
	} else { //In the easter egg pattern, each robot is on its own cell.
		let mut i = 0;
		'step: loop {
			for robot in &mut robots {
				robot.position.0 += robot.velocity.0;
				robot.position.1 += robot.velocity.1;
				while robot.position.0 >= WIDTH  { robot.position.0 -= WIDTH  };
				while robot.position.0 < 0       { robot.position.0 += WIDTH  };
				while robot.position.1 >= HEIGHT { robot.position.1 -= HEIGHT };
				while robot.position.1 < 0       { robot.position.1 += HEIGHT };
			}

			i += 1;
			println!("{i}");
			
			let mut g = ConstSizeGrid::<WIDTH_U, HEIGHT_U, bool>::filled_with(false);
			for robot in &robots {
				if *g.get(robot.position.0 as usize, robot.position.1 as usize) {
					continue 'step;
				}
				g.set(robot.position.0 as usize, robot.position.1 as usize, true);
			}
			// If we didn't go to the next step, there were no collisions.
			return Ok(format!("{i}"));
		}
	}
}
