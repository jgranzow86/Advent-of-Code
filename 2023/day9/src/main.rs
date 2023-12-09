use std::{
    fs,
    str::FromStr,
    time::Instant,
};

fn main() {
    let now = Instant::now();
    let input = fs::read_to_string("input.txt").expect("Error reading file");

    let data = parse_input(&input);
    let elapsed = now.elapsed();
    println!("{}us to parse", elapsed.as_micros());

    part1(&data);

    part2(&data);
    let elapsed2 = now.elapsed();
    println!("{}us Total", elapsed2.as_micros());
}

fn part1(data: &Vec<Vec<i64>>) -> i64{
    let now = Instant::now();
    let mut answer = 0;
    for line in data {
        answer += figure_it_out_bud(&line)
    }
    let elapsed = now.elapsed();
    println!("({}μs)Part 1: {}", elapsed.as_micros(), answer);
    answer
}

fn part2(data: &Vec<Vec<i64>>) -> i64 {
    let now = Instant::now();
    let mut answer = 0;
    for line in data {
        answer += figure_it_out_bud_part_2(&line)
    }
    let elapsed = now.elapsed();
    println!("({}μs)Part 2: {}", elapsed.as_micros(), answer);
    answer
}

fn figure_it_out_bud(data: &Vec<i64>) -> i64{
    if !data.iter().all(|x| *x == 0) {
        let next_data = data
            .windows(2)
            .map(|item| item[1] - item[0])
            .collect::<Vec<i64>>();
        
        figure_it_out_bud(&next_data) + data
            .last()
            .unwrap()
    } else {
        0
    }
}

fn figure_it_out_bud_part_2(data: &Vec<i64>) -> i64{
    if !data.iter().all(|x| *x == 0) {
        let next_data = data
            .windows(2)
            .map(|item| item[0] - item[1])
            .collect::<Vec<i64>>();
        
        figure_it_out_bud_part_2(&next_data) + data
            .first()
            .unwrap()
    } else {
        0
    }
}

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
    .lines()
    .map(|line| line.split(" ")
        .map(|item| item.parse::<i64>()
        .unwrap())
        .collect::<Vec<_>>())
    .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        
        let data = parse_input(&input);
        let answer = part1(&data);
        assert_eq!(answer, 114);
    }

    #[test]
    fn test_part_2() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        let data = parse_input(&input);
        let answer = part2(&data);
        assert_eq!(answer, 2);
    }
}