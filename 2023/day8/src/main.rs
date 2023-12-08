use std::{
    fs,
    str::FromStr,
    time::Instant,
};
use nom::{
    Finish,
    IResult,
    sequence::tuple,
    character::complete::{
        alpha1,
        space1,
        anychar
    }
};

fn main() {
    let input= fs::read_to_string("input.txt").expect("Error reading file");

    let now = Instant::now();
    let part1_answer = part1(&input);
    let elapsed = now.elapsed();
    println!("({}μs)Part 1: {}", elapsed.as_micros(), part1_answer);


    let now = Instant::now();
    let part2_answer = part2(&input);
    let elapsed = now.elapsed();
    println!("Performance Report:");
    println!("({}us)Part 2: {}", elapsed.as_nanos(), part2_answer);
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

    loop {
        for direction in directions.chars() {
            if direction == 'L' {
                current_node = nodes.iter().find(|node| current_node.left == node.source).unwrap();
            } else if direction == 'R'{
                current_node = nodes.iter().find(|node| current_node.right == node.source).unwrap();
            } else {
                todo!();
            }

            step_count += 1;
            if current_node.source == "ZZZ" {
                break;
            }
        }
        if current_node.source == "ZZZ" {
            break;
        }
    }
    step_count
}

fn part2(input: &String) -> u64 {
    
    0
}

#[derive(Clone, Default)]
struct Node {
    source: String,
    left: String,
    right: String,
}

impl Node {
    fn new(source: &str, left: &str, right: &str) -> Node {
        let source = String::from_str(source).unwrap();
        let left = String::from_str(left).unwrap();
        let right = String::from_str(right).unwrap();
        Node { source, left, right }
    }
}

fn parse_nodes(input: &str) -> IResult<&str, (&str, &str, &str)> {
    let (input, (source, _, _, _, _, left, _, _, right, _)) = 
        tuple((alpha1, space1, anychar, space1, anychar, alpha1, anychar, space1, alpha1, anychar))(input)?;

    Ok((input, (source, left, right)))
}

#[cfg(test)]
mod tests {
    use nom::Finish;

    use super::*;
    #[test]
    fn test_part_1() {
        let input = fs::read_to_string("sample.txt").expect("Error reading file");
        

        let answer = part1(&input);
        assert_eq!(answer, 6);
    }

    #[test]
    fn test_part_2() {
        let input = fs::read_to_string("sample.txt").expect("Error reading file");

        let answer = part2(&input);
        assert_eq!(answer, 30);
    }

    #[test]
    fn test_parse_nodes() {
        let input = fs::read_to_string("sample.txt").expect("Error reading files");

        let mut nodes = Vec::new();

        for line in input.lines().skip(2) {
            let (_consumed_input, (source, left, right)) = parse_nodes(line).finish().unwrap();
            nodes.push(Node::new(source, left, right));
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