use std::{fs, time::Instant};

extern crate humantime;
use humantime::format_duration;

#[macro_use]
extern crate prettytable;

fn main() {
	let input = fs::read_to_string("day3/input.txt").expect("Error reading file");

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

#[derive(Clone, Debug)]
struct Bank {
	batteries: Vec<u32>,
}

impl Bank {
	fn new(batteries_str: &str) -> Self {
		let batts = batteries_str
			.chars()
			.map(|c| c.to_digit(10).unwrap())
			.collect::<Vec<u32>>();

		Self { batteries: batts }
	}

	fn get_bank_joltage(&self, battery_count: u32) -> u64 {
		let total = Self::get_bank_joltage_string(&self.batteries, battery_count)
			.parse::<u64>()
			.unwrap();
		total
	}

	fn get_bank_joltage_string(bank: &[u32], battery_count: u32) -> String {
		let mut largest_battery = 0;
		for batt in 0..(bank.len() - (battery_count as usize - 1)) {
			if bank[largest_battery] < bank[batt] {
				largest_battery = batt;
			}
		}

		let new_battery_count = battery_count - 1;
		if new_battery_count <= 0 {
			return bank[largest_battery].to_string();
		}

		let bank_slice = &bank[(largest_battery + 1)..];
		let battery_string =
			Self::get_bank_joltage_string(bank_slice, new_battery_count).to_string();

		let battery_string = bank[largest_battery].to_string() + &battery_string;

		battery_string
	}
}

fn part1(input: &str) -> isize {
	let mut accumulator = 0;

	let banks = input
		.lines()
		.map(|line| Bank::new(line))
		.collect::<Vec<Bank>>();
	for bank in banks {
		accumulator += bank.get_bank_joltage(2);
	}

	accumulator as isize
}

fn part2(input: &str) -> isize {
	let mut accumulator = 0;

	let banks = input
		.lines()
		.map(|line| Bank::new(line))
		.collect::<Vec<Bank>>();
	for bank in banks {
		accumulator += bank.get_bank_joltage(12);
	}

	accumulator as isize
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_part1() {
		let input = fs::read_to_string("day3/example.txt").expect("Error reading file");
		let answer = part1(&input);
		assert_eq!(answer, 357);
	}
	#[test]
	fn test_part2() {
		let input = fs::read_to_string("day3/example.txt").expect("Error reading file");
		let answer = part2(&input);
		assert_eq!(answer, 3121910778619);
	}
}
