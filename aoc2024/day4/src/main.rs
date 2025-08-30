use std::{fs, process::Output, time::Instant};
extern crate humantime;
use humantime::format_duration;

extern crate nom;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{anychar, char, i64},
    multi::many_till,
    sequence::tuple,
    Finish, IResult,
};

#[macro_use]
extern crate prettytable;

fn main() {
    // let input = fs::read_to_string("day4/input.txt").expect("Error reading file");
    let input = fs::read_to_string("day4/example.txt").expect("Error reading file");

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

fn part1(input: &str) -> isize {
    let positions: Vec<(i64, i64, char)> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, value)| (x as i64, y as i64, value))
        })
        .collect();

    let mut x_pos = Vec::new();
    for each in positions {
        if each.2 == 'X' {
            x_pos.push((each.0, each.1, each.2));
        }
    }

    for each in x_pos {
        let (x, y, c) = each;
        println!("X: {x}, Y: {y}, Value: {c}");
    }
    0
}

fn part2(input: &str) -> isize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let input = fs::read_to_string("example.txt").expect("Error reading file");
        let result = part1(&input);
        assert_eq!(161, result);
    }

    #[test]
    fn test_part2() {
        assert!(true);
    }
}
