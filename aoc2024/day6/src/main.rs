use std::{
    collections::{hash_map, HashMap},
    fs,
    time::Instant,
};
extern crate humantime;
use humantime::format_duration;

extern crate nom;
// use nom::{
//     branch::alt,
//     bytes::complete::{tag, take_until},
//     character::complete::{anychar, char, i64},
//     multi::many_till,
//     sequence::tuple,
//     Finish, IResult,
// };

#[macro_use]
extern crate prettytable;

fn main() {
    // let input = fs::read_to_string("day6/input.txt").expect("Error reading file");
    let input = fs::read_to_string("day6/example.txt").expect("Error reading file");

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
    let mut map = parse_map(&input);
    let mut Guard = Guard::find_guard(&map).unwrap();
    
    while Guard.inbounds

    0
}

fn part2(input: &str) -> isize {
    0
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Key {
    BreadCrumb,
    Obstacle,
    Open,
    Guard(Direction),
}

impl Key {
    fn resolve(ch: char) -> Key {
        match ch {
            'X' => Self::BreadCrumb,
            '#' => Self::Obstacle,
            '.' => Self::Open,
            '^' => Self::Guard(Direction::North),
            'v' => Self::Guard(Direction::South),
            '>' => Self::Guard(Direction::East),
            '<' => Self::Guard(Direction::West),
            _ => panic!("Unknown map item"),
        }
    }
}

struct Guard {
    x: isize,
    y: isize,
    facing: Direction,
}

#[derive(Debug)]
enum Error {
    GuardNotFound,
}

impl Guard {
    fn find_guard(map: &HashMap<(isize, isize), Key>, x_max: &isize, y_max: &isize) -> Result<Self, Error> {
        for item in map {
            let ((x, y), key) = item;
            if let Key::Guard(facing) = key {
                let a = x.clone();
                let b = y.clone();
                let c = facing.clone();
                return Ok(Guard { x: a, y: b, facing: c })
            }
        }
        Err(Error::GuardNotFound)
    }
}

fn parse_map(input: &str) -> HashMap<(isize, isize), Key> {
    let mut map: HashMap<(isize, isize), Key> = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let value = Key::resolve(ch);
            map.insert((x as isize, y as isize), value);
        }
    }
    map
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
    fn test_parse_map() {
        let input = fs::read_to_string("example.txt").expect("Error reading file");
        let map = parse_map(&input);
        let test1 = map.get_key_value(&(0, 0));
        let test2 = map.get_key_value(&(4, 0));
        let test3 = map.get_key_value(&(4, 6));
        
        assert_eq!(Some((&(0, 0), &Key::Open)), test1);
        assert_eq!(Some((&(4, 0), &Key::Obstacle)), test2);
        assert_eq!(Some((&(4, 6), &Key::Guard(Direction::North))), test3);

    }
}
