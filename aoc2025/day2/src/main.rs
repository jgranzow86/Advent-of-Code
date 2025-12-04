use std::{fs, time::Instant};

extern crate humantime;
use humantime::format_duration;

#[macro_use]
extern crate prettytable;

fn main() {
	let input = fs::read_to_string("day2/input.txt").expect("Error reading file");

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

struct IDRange {
	lower: isize,
	upper: isize,
}

impl IDRange {
	fn new(id_range: &str) -> Self {
		let range: Vec<isize> = id_range
			.split('-')
			.map(|s| s.parse::<isize>().unwrap())
			.collect();

		Self {
			lower: range[0],
			upper: range[1],
		}
	}
}

fn part1(input: &str) -> isize {
	let input = input.lines().next().unwrap();
	let input: Vec<&str> = input.split(',').collect();
	let mut id_ranges: Vec<IDRange> = Vec::new();
	let mut accumulator = 0;

	input
		.iter()
		.for_each(|range| id_ranges.push(IDRange::new(range)));

	for ids in id_ranges {
		for id in ids.lower..=ids.upper {
			let id_string = id.to_string();
			let half_mark = id_string.len() / 2;
			let (f_half, s_half) = id_string.split_at(half_mark);

			if f_half == s_half {
				accumulator += id
			}
		}
	}

	accumulator
}

fn all_elements_equal(vector: Vec<String>) -> bool {
	if vector.is_empty() {
		return true;
	}

	let first_element = &vector[0];
	vector.iter().all(|element| element == first_element)
}

fn get_factors(number: usize) -> Vec<usize> {
	let mut factors: Vec<usize> = Vec::new();

	for i in 1..number {
		let rem = number % i;
		if rem == 0 {
			factors.push(i);
		}
	}
	factors
}

fn part2(input: &str) -> isize {
	let input = input.lines().next().unwrap();
	let input: Vec<&str> = input.split(',').collect();
	let mut id_ranges: Vec<IDRange> = Vec::new();
	let mut accumulator = 0;

	input
		.iter()
		.for_each(|range| id_ranges.push(IDRange::new(range)));

	for ids in id_ranges {
		for id in ids.lower..=ids.upper {
			let id_string = id.to_string();

			for each in get_factors(id_string.len()) {
				let slice = id_string.chars().collect::<Vec<char>>();
				let chunk = slice
					.chunks(each)
					.map(|c| c.iter().collect::<String>())
					.collect::<Vec<String>>();

				if all_elements_equal(chunk) {
					accumulator += id;
					break;
				}
			}
		}
	}

	accumulator
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_part1() {
		let input = fs::read_to_string("day2/example.txt").expect("Error reading file");
		let answer: isize = part1(&input);
		assert_eq!(answer, 1227775554);
	}
	#[test]
	fn test_part2() {
		let input = fs::read_to_string("day2/example.txt").expect("Error reading file");
		let answer: isize = part2(&input);
		assert_eq!(answer, 4174379265);
	}
}
