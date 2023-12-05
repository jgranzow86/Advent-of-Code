use std::str::FromStr;

fn main() {
    let input = include_str!("input.txt");

    part1(input);
}

struct Number {
    num: i32,
    x: i32,
    y: i32,
}

fn part1(input: &str) -> u64 {
    let mut part_numbers = Vec::new();
    
    let found: Vec<Number> = Vec::new();

    let mut lines = input.lines();

    let mut x = 0;
    let mut y = 0;

    for (y, line) in lines.enumerate() {
        for (x, c) in line.chars().enumerate() {
            println!("{0},{1} {2}", x, y, c);
        }
    }


    part_numbers.push(8);
    let mut answer = 0;
    for part_number in part_numbers {
        answer += part_number;
    }
    answer // dummy return
}

fn is_symbol(item: char) -> bool {
    if !item.is_digit(10) && !item.is_alphabetic() && item != '.'{
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let sample: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let sum = part1(&sample);
        assert_eq!(sum, 142);
    }

    #[test]
    fn test_is_symbol() {
        let samples = vec!['a', '1', '.'];
        for sample in samples {
            assert!(!is_symbol(sample));
        }

        let samples = vec!['!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '_', '=', '+', '/'];
        for sample in samples {
            assert!(is_symbol(sample));
        }
    }
}