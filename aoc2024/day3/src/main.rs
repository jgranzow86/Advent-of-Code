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
    let input = fs::read_to_string("day3/input.txt").expect("Error reading file");
    // let input = fs::read_to_string("day3/example.txt").expect("Error reading file");

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

#[derive(Debug)]
enum Instruction {
    Mul(i64, i64),
    Do,
    Dont,
}

fn part1(input: &str) -> isize {
    let data = parse(input);
    data.iter()
        .map(|i| {
            if let Instruction::Mul(x, y) = i {
                *x as isize * *y as isize
            } else {
                0
            }
        })
        .sum()
}

fn part2(input: &str) -> isize {
    let data = parse(input);
    let mut do_flag: bool = true;
    data.iter()
        .map(|i| match i {
            Instruction::Do => {
                do_flag = true;
                0
            }
            Instruction::Dont => {
                do_flag = false;
                0
            }
            Instruction::Mul(x, y) => {
                if do_flag {
                    *x as isize * *y as isize
                } else {
                    0
                }
            }
        })
        .sum()
}

fn instruction_parser(input: &str) -> IResult<&str, Instruction> {
    let (input, (_, x, _, y, _)) = tuple((tag("mul("), i64, char(','), i64, char(')')))(input)?;
    Ok((input, Instruction::Mul(x, y)))
}

fn do_parser(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("do()")(input)?;
    Ok((input, Instruction::Do))
}

fn dont_parser(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("don't()")(input)?;
    Ok((input, Instruction::Dont))
}

fn multi_parser(input: &str) -> IResult<&str, Instruction> {
    alt((instruction_parser, do_parser, dont_parser))(input)
}

fn parse(input: &str) -> Vec<Instruction> {
    let mut input2 = input;
    let mut instructions = Vec::new();
    while !input2.is_empty() {
        let out = many_till(anychar, multi_parser)(input2).finish();
        let Ok((input, (_, instruction))) = out else {
            break;
        };
        input2 = input;
        instructions.push(instruction);
    }
    instructions
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
