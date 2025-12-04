use std::{fmt, fs, time::Instant};

extern crate humantime;
use humantime::format_duration;

#[macro_use]
extern crate prettytable;

fn main() {
	let input = fs::read_to_string("day1/input.txt").expect("Error reading file");

	let now_part1 = Instant::now();
	let answer_part1 = part1(&input);
	let elapsed_part1 = now_part1.elapsed();
	let humantime_elapsed_part1 = format_duration(elapsed_part1).to_string();

	let now_part2 = Instant::now();
	let answer_part2 = part2(&input);
	let elapsed_part2 = now_part2.elapsed();
	let humantime_elapsed_part2 = format_duration(elapsed_part2).to_string();

	let elapsed_total = elapsed_part1 + elapsed_part2;
	let humantime_elapsed_total = format_duration(elapsed_total).to_string();

	ptable!(
		["Task", "Run Time", "Answer"],
		["Part 1", humantime_elapsed_part1, answer_part1],
		["Part 2", humantime_elapsed_part2, answer_part2],
		["Total", humantime_elapsed_total, "-"]
	);
}

#[derive(PartialEq)]
enum Direction {
	Left,
	Right,
}

impl fmt::Display for Direction {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Direction::Left => write!(f, "Left"),
			Direction::Right => write!(f, "Right"),
		}
	}
}

impl Direction {
	fn from_str(s: &str) -> Result<Self, &str> {
		match s.to_lowercase().as_str() {
			"left" | "l" => Ok(Direction::Left),
			"right" | "r" => Ok(Direction::Right),
			_ => Err("Unknown Direction"),
		}
	}
}

struct Dial {
	position: isize,
	total_zeros: isize,
}

impl Dial {
	fn left(&mut self, left_rotation: isize) {
		for _ in 0..left_rotation {
			self.position -= 1;
			if self.position == 0 {
				self.total_zeros += 1;
			}
			if self.position == -1 {
				self.position = 99;
			}
		}
	}

	fn right(&mut self, right_rotation: isize) {
		for _ in 0..right_rotation {
			self.position += 1;
			if self.position == 100 {
				self.position = 0;
				self.total_zeros += 1;
			}
		}
	}

	fn parse_direction(&mut self, spin: &str) {
		let mut c = spin.chars();

		let direction = Direction::from_str(&c.next().unwrap().to_string()).unwrap();

		if direction == Direction::Left {
			let rot = c.as_str().parse::<isize>().unwrap();
			self.left(rot);
		} else {
			let rot = c.as_str().parse::<isize>().unwrap();
			self.right(rot);
		}
	}
}

fn part1(input: &str) -> isize {
	let mut zero_count = 0;
	let mut lock = Dial {
		position: 50,
		total_zeros: 0,
	};
	for line in input.lines() {
		lock.parse_direction(line);
		if lock.position == 0 {
			zero_count += 1;
		}
	}

	zero_count
}

fn part2(input: &str) -> isize {
	let mut lock = Dial {
		position: 50,
		total_zeros: 0,
	};
	for line in input.lines() {
		lock.parse_direction(line);
	}

	lock.total_zeros

	//7106 too high
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_part1() {
		let input = fs::read_to_string("example.txt").expect("Error reading file");
		let answer: isize = part1(&input);
		assert_eq!(answer, 3);
	}
	#[test]
	fn test_part2() {
		let input = fs::read_to_string("example.txt").expect("Error reading file");
		let answer: isize = part2(&input);
		assert_eq!(answer, 6);
	}
}
