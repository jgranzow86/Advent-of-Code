fn main() {
    let input = include_str!("input.txt");

    let answer: i32 = part1(&input);
    println!("Part 1: {answer}");

    let answer: i32 = part2(&input);
    println!("Part 2: {answer}");
}

fn part1(input: &str) -> i32 {
    let mut sum: i32 = 0;

    for line in input.lines() {
        let mut numbers: String = String::new();
        // find  first number
        numbers.push(line.chars()
            .find(|x| x.is_digit(10)).unwrap());
        // find last number if none return the first number
        numbers.push(line.chars().rfind(|x| x.is_digit(10))
            .unwrap_or(numbers
                .chars()
                .next()
                .unwrap()));

        sum += numbers.parse::<i32>().unwrap();
    }

    sum
}

fn part2(input: &str) -> i32 {
    let new_input = input.replace("one", "o1e")
        .replace("two", "t2o" )
        .replace("three", "t3e")
        .replace("four", "4")
        .replace("five", "5e")
        .replace("six", "6")
        .replace("seven", "7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");

    part1(new_input.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let sample: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        let sum: i32 = part1(&sample);
        assert_eq!(sum, 142);
    }
    #[test]
    fn test_part_2() {
        let sample: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let sum = part2(&sample);
        assert_eq!(sum, 281)
    }
}