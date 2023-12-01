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
        numbers.push(line.chars()
            .find(|x| x.is_digit(10)).unwrap());
        numbers.push(line.chars().rfind(|x| x.is_digit(10))
            .unwrap_or(numbers
                .chars()
                .next()
                .unwrap()));

        sum += numbers.parse::<u32>().unwrap();
    }

    sum
}

fn part2(input: &str) -> String {
    todo!()
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

    // #[test]
    // fn test2() {
    //     let sample: &str = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen"
    // }
}