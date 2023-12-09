use std::{
    fs,
    str::FromStr,
    time::Instant, ops::Deref,
};
use nom::{
    Finish,
    IResult,
    sequence::tuple,
    character::complete::{
        space1,
        anychar,
        alphanumeric1
    }
};
use derivative::Derivative;
use num::integer::lcm;

fn main() {
    let input= fs::read_to_string("input.txt").expect("Error reading file");

    let now = Instant::now();
    let part1_answer = part1(&input);
    let elapsed = now.elapsed();
    println!("({}μs)Part 1: {}", elapsed.as_micros(), part1_answer);


    let now = Instant::now();
    let part2_answer = part2(&input);
    let elapsed = now.elapsed();
    println!("({}μs)Part 2: {}", elapsed.as_nanos(), part2_answer);
}

fn part1(input: &str) -> u64 {
    let directions = input.lines().nth(0).unwrap();
    let mut nodes = Vec::new();
    let mut step_count =  0;


    for line in input.lines().skip(2) {
        let (_consumed_input, (source, left, right)) = parse_nodes(line).finish().unwrap();
        nodes.push(Node::new(source, left, right));
    }

        let mut current_node = nodes
            .iter()
            .find(|node| node.source == "AAA")
            .unwrap();

    'outer: loop {
        for direction in directions.chars() {
            current_node = nodes.iter().find(|node| {
                match direction {
                    'L' => current_node.left == node.source,
                    'R' => current_node.right == node.source,
                    _ => panic!("This shouldn't happen!!!"),
                }
            }).unwrap();

            step_count += 1;
            if current_node.source == "ZZZ" {
                break 'outer;
            }
        }
    }
    step_count
}

fn part2(input: &str) -> u64 {
    let directions = input.lines().nth(0).unwrap();
    let mut nodes = Vec::new();
    let mut step_count =  0;
    let mut starting_nodes = Vec::new();
    let mut step_counts = Vec::new();


    for line in input.lines().skip(2) {
        let (_consumed_input, (source, left, right)) = parse_nodes(line).finish().unwrap();
        let source = source.to_string();
        let left = left.to_string();
        let right = right.to_string();
        let zero_step = 0;
        nodes.push(Node { source, left, right, step_count: zero_step });
    }

    for node in nodes.as_slice() {
        if node.source.chars().last() == Some('A') {
            starting_nodes.push(node);
        }
    }

    for start_node in starting_nodes {
        let mut current_node: Node = start_node.clone();
        step_count = 0;
        'outer: loop {
            for direction in directions.chars() {
                current_node = nodes.iter().find(|node| {
                    match direction {
                        'L' => current_node.left == node.source,
                        'R' => current_node.right == node.source,
                        _ => panic!("This shouldn't happen!!!"),
                    }
                }).unwrap().clone();

                step_count += 1;
                if current_node.source.chars().last() == Some('Z') {
                    break 'outer;
                }
            }
        }
        step_counts.push(step_count);
    }
    step_counts.iter().fold(1, |acc, &x| lcm(acc, x))
}

#[derive(Derivative)]
#[derive(Clone, Default)]
struct Node {
    source: String,
    left: String,
    right: String,
    #[derivative(Default(value = "0"))]
    step_count: u64,
}

// #[derive(Clone)]
// struct Starts {
//     current: Node,
//     step_count: u64,
// }

impl Node {
    fn new(source: &str, left: &str, right: &str) -> Node {
        let source = String::from_str(source).unwrap();
        let left = String::from_str(left).unwrap();
        let right = String::from_str(right).unwrap();
        let step_count = 0;
        Node { source, left, right, step_count }
    }
}

fn parse_nodes(input: &str) -> IResult<&str, (&str, &str, &str)> {
    let (input, (source, _, _, _, _, left, _, _, right, _)) = 
        tuple((alphanumeric1, space1, anychar, space1, anychar, alphanumeric1, anychar, space1, alphanumeric1, anychar))(input)?;

    Ok((input, (source, left, right)))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let input = fs::read_to_string("sample.txt").expect("Error reading file");
        

        let answer = part1(&input);
        assert_eq!(answer, 6);
    }

    #[test]
    fn test_part_2() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        let answer = part2(&input);
        assert_eq!(answer, 6);
    }

    #[test]
    fn test_parse_nodes() {
        let input = fs::read_to_string("sample.txt").expect("Error reading files");

        let mut nodes = Vec::new();

        for line in input.lines().skip(2) {
            let (_consumed_input, (source, left, right)) = parse_nodes(line).finish().unwrap();
            let source = source.to_string();
            let left = left.to_string();
            let right = right.to_string();
            let step_count = 0;
            nodes.push(Node { source, left, right, step_count });
        }

        let mut node = nodes.iter();

        let x = node.next().unwrap().clone();
        assert_eq!((x.source, x.left, x.right), (String::from("AAA"), String::from("BBB"), String::from("BBB")));
        let x = node.next().unwrap().clone();
        assert_eq!((x.source, x.left, x.right), (String::from("BBB"), String::from("AAA"), String::from("ZZZ")));
        let x = node.next().unwrap().clone();
        assert_eq!((x.source, x.left, x.right), (String::from("ZZZ"), String::from("ZZZ"), String::from("ZZZ")));
    }
}