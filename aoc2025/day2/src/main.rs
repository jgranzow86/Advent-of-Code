use std::{fs, time::Instant};

extern crate humantime;
use humantime::format_duration;

#[macro_use]
extern crate prettytable;

fn main() {
    let input = fs::read_to_string("day1/input.txt").expect("Error reading file");
    // let input = fs::read_to_string("day1/example.txt").expect("Error reading file");

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

fn part1(input: &str) -> usize {
    let _ = input;
    1
}

fn part2(input: &str) -> usize {
    let _ = input;
    2
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let answer: isize = 0;
        assert_eq!(answer, 0);
    }
}