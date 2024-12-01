use std::{
    fs, time::Instant
};
extern crate humantime;
use humantime::format_duration;
extern crate nom;
use nom::{
    Finish,
    IResult,
    sequence::tuple,
    character::complete::{space1, digit1}
};
use std::collections::HashMap;

#[macro_use] extern crate prettytable;

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

    ptable!(["Task", "Run Time", "Answer"],
            ["Part 1", humantime_elapsed_part1, answer_part1],
            ["Part 2", humantime_elapsed_part2, answer_part2],
            ["Total", humantime_elapsed_total, "-"]);
}

fn part1(input: &str) -> usize {
    let (list1, list2) = parse_list(&input);

    let mut distance = Vec::with_capacity(1000);

    for each in 0..list1.len() {
        let diff = list1[each] as isize - list2[each] as isize;
        distance.push(diff.abs() as usize);
    }
    distance.iter().sum()
}

fn part2(input: &str) -> usize {
    let (list1, list2) = parse_list(&input);
    let mut sum: usize = 0;

    for x in list1 {
        let count = list2.iter().filter(|y| x == **y).count();
        sum += x * count;
    }
    sum
}

fn parse_list(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut list1: Vec<usize> = Vec::with_capacity(1000);
    let mut list2: Vec<usize> = Vec::with_capacity(1000);
    for line in input.lines() {
        let (_, (num1, num2)) = parse_numbers(line).finish().unwrap();
        list1.push(num1);
        list2.push(num2);
    }
    list1.sort();
    list2.sort();

    (list1, list2)
}

fn parse_numbers(input: &str) -> IResult<&str, (usize, usize)> {
    let (_, (item1, _, item2)) = tuple((digit1, space1, digit1))(input)?;

    let num1 = item1.parse::<usize>().unwrap();
    let num2 = item2.parse::<usize>().unwrap();

    Ok((input, (num1, num2)))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_number() {
        let input = "76569   66648".to_string();
        let (_, (num1, num2)) = parse_numbers(&input).finish().unwrap();
        assert_eq!(num1, 76569);
        assert_eq!(num2, 66648);
    }

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string("example.txt").expect("Error reading file");
        let answer = part1(&input);
        assert_eq!(11, answer);
    }

    #[test]
    fn test_part_2() {
        let input = fs::read_to_string("example.txt").expect("Error reading file");
        let answer = part2(&input);
        assert_eq!(31, answer);
    }
}