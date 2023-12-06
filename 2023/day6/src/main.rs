use std::{str::FromStr};

fn main() {
    let input = String::from_str(include_str!("input.txt")).unwrap();

    let part1_answer = part1(&input);
    println!("Part 1: {part1_answer}");

    let part2_answer = part2(&input);
    println!("Part 2: {part2_answer}")
}

fn part1(input: &String) -> u64 {
    let races = parse_input(&input);

    let mut answer = 1;
    let mut win_counts = Vec::new();
    
    for race in races {
        let mut won_races = 0;
        for button_held in 0..=race.time {
            let remaining_time = race.time - button_held;
            let distance_traveled = button_held * remaining_time;
            if distance_traveled > race.distance {
                won_races += 1;
            }
        }
        win_counts.push(won_races);
    }
    for each in win_counts {
        answer = answer * each;
    }
    answer
}

fn part2(input: &String) -> u64 {
    let input2 = input.clone();
    let answer = 0;

    answer
}

fn parse_input(input: &String) -> Vec<Race> {
    let mut time_collection = Vec::new();
    let mut distance_collection = Vec::new();
    let mut races = Vec::new();

    for line in input.lines() {
        if line.starts_with("Time:") {
            time_collection = line
                .strip_prefix("Time:")
                .unwrap()
                .split(" ")
                .flat_map(&str::parse)
                .collect::<Vec<u64>>();
        } else if line.starts_with("Distance:") {
            distance_collection = line
                .strip_prefix("Distance:")
                .unwrap()
                .split(" ")
                .flat_map(&str::parse)
                .collect::<Vec<u64>>();
        }
    }
    let mut race_index = 0;
    while race_index < time_collection.len() {
        let time = time_collection[race_index];
        let distance = distance_collection[race_index];
        races.push(Race{ time, distance });
        race_index += 1;
    }
    races
}

struct Race {
    time: u64,
    distance: u64,
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let input = String::from_str(include_str!("sample.txt")).unwrap();
        

        let answer = part1(&input);
        assert_eq!(answer, 288);
    }

    #[test]
    fn test_part_2() {
        let input = String::from_str(include_str!("sample.txt")).unwrap();

        let answer = part2(&input);
        assert_eq!(answer, 30);
    }

    #[test]
    fn test_parse_input() {
        let input = String::from_str(include_str!("sample.txt")).unwrap();
        let expected_output = vec![Race { time: 7, distance: 9 },
            Race { time: 15, distance: 40 },
            Race { time: 30, distance: 200 }];
        let output = parse_input(&input);
        for index in 0..=2 {
            assert_eq!(expected_output[index].time, output[index].time);
            assert_eq!(expected_output[index].distance, output[index].distance);
        }
    }
}