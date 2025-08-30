use std::{
    collections::{hash_map, HashMap},
    fs,
    time::Instant,
};
extern crate humantime;
use humantime::format_duration;
use nom::InputIter;

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
    // let input = fs::read_to_string("day9/input.txt").expect("Error reading file");
    let input = fs::read_to_string("day9/example.txt").expect("Error reading file");

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
    for e in (0..10).rev() {
        println!("{e}");
    }
    let files = parse_input(&input);
    let mut disk = Vec::new();

    for file in &files {
        for _ in 0..file.size {
            disk.push(Block::Occupied(file.id.clone()));
        }
        for _ in 0..file.free {
            disk.push(Block::Vacant);
        }
    }
    println!("Break");
    0
}

fn part2(input: &str) -> isize {
    0
}

#[derive(Clone, Copy)]
enum Block {
    Occupied(usize),
    Vacant,
}

#[derive(Clone, Copy)]
struct File {
    id: usize,
    size: usize,
    free: usize,
}

fn parse_input(input: &str) -> Vec<File> {
    let mut input = input;
    let mut files: Vec<File> = Vec::new();
    let mut id_counter = 0;

    while input.len() > 1 {
        let (output, (file_size_opt, free_space_opt)) = parse_files(input).finish().unwrap();
        let file_size = file_size_opt.unwrap();
        let free_space: u32;
        if let Some(free_space_opt) = free_space_opt {
            free_space = free_space_opt;
        } else {
            break;
        }
        files.push(File {
            id: id_counter,
            size: file_size as usize,
            free: free_space as usize,
        });

        id_counter += 1;
        input = output;
    }

    let last = input.parse::<u32>().unwrap();

    files.push(File {
        id: id_counter,
        size: last as usize,
        free: 0,
    });

    files
}

fn parse_files(input: &str) -> IResult<&str, (Option<u32>, Option<u32>)> {
    let (input, (file_size, free_space)) = tuple((anychar, anychar))(input)?;
    let file_size = file_size.to_digit(10);
    let free_space = free_space.to_digit(10);

    Ok((input, (file_size, free_space)))
}

fn is_even(int: isize) -> bool {
    let shifted = int & 1;
    if shifted == 0 {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let input = fs::read_to_string("example.txt").expect("Error reading file");
        let result = part1(&input);
        assert_eq!(41, result);
    }

    #[test]
    fn test_part2() {
        assert!(true);
    }

    #[test]
    fn test_is_even() {
        assert!(is_even(8));
        assert!(is_even(153242));
        assert!(!is_even(3));
        assert!(!is_even(09834757389));
    }
}
