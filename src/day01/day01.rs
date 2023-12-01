use std::str::FromStr;

fn main() {
    let input = include_str!("input.txt");

    let answer: u32 = part1(input);
    println!("{answer}");
}

fn part1(input: &str) -> u32 {
    let mut sum: u32 = 0;

    for line in input.lines() {
        let mut numbers: String = String::new();
        for ch in line.chars() {
            if ch.is_digit(10) {
                numbers.push(ch);
            }
        }
        
        if numbers.len() > 2 {
            let mut chars = numbers.chars();
            let x = chars.next().unwrap();
            let y = chars.next_back().unwrap();
            numbers = x.to_string();
            numbers.push(y);
        }

        if numbers.len() < 2 {
            numbers.push(numbers.clone().chars().next().unwrap());
        }

        sum += numbers.parse::<u32>().unwrap();
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let sample: &str = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";

        let sum: u32 = part1(sample);
        assert_eq!(sum, 142);
    }

    #[test]
    fn test2() {
        let sample: &str = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen"
    }
}