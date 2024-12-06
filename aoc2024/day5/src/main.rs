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
    let input = fs::read_to_string("day5/input.txt").expect("Error reading file");
    // let input = fs::read_to_string("day5/example.txt").expect("Error reading file");

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
    let mut rules: HashMap<isize, Vec<isize>> = HashMap::new();

    // let mut rules = Vec::new();
    let mut pages_list: Vec<Vec<isize>> = Vec::new();
    for line in input.lines() {
        if line.contains("|") {
            // rules.push(Rule::new(&line));
            let (before, after) = split_rules(line);
            if rules.contains_key(&before) {
                let afters = rules.get_mut(&before).unwrap();
                afters.push(after);
            } else {
                rules.insert(before, vec![after]);
            }
        } else if line.contains(",") {
            pages_list.push(
                line.split(',')
                    .map(|x| x.parse::<isize>().unwrap())
                    .collect(),
            );
        }
    }

    let mut page_outcome = Vec::new();

    for pages in pages_list {
        let mut is_bad = false;
        let mut buffer = Vec::new();
        for page in &pages {
            if let Some(rls) = rules.get(&page) {
                for rule in rls {
                    if buffer.contains(rule) {
                        is_bad = true;
                        break;
                    }
                }
            }
            buffer.push(page.clone()); 
        }
        if is_bad {
            page_outcome.push(Order::Incorrect(pages.clone()));
        } else {
            page_outcome.push(Order::Correct(pages.clone()));
        }
    }

    let mut sum = 0;
    for each in page_outcome {
        if let Order::Correct(cpages) = each {
            sum += Order::Correct(cpages).get_middle_value();
        }
    }
    sum
}

fn part2(input: &str) -> isize {
    0
}

fn split_rules(raw_rule: &str) -> (isize, isize) {
    let split_rule: Vec<&str> = raw_rule.split('|').collect();
    let left = split_rule[0].parse::<isize>().unwrap();
    let right = split_rule[1].parse::<isize>().unwrap();
    (left, right)
}

enum Order {
    Correct(Vec<isize>),
    Incorrect(Vec<isize>),
}

impl Order {
    fn get_middle_value(self) -> isize {
        match self {
            Self::Correct(update) => {
                let middle_position = update.len() / 2;
                update[middle_position]
            }
            Self::Incorrect(update) => {
                let middle_position = update.len() / 2;
                update[middle_position]
            }
        }
    }
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

    #[test]
    fn test_order() {
        let v = Order::Correct(vec![1, 2, 3, 4, 5]);
        assert_eq!(3, v.get_middle_value());
    }
}
