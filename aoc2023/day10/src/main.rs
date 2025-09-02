use core::panic;
use std::{
    fs,
    time::Instant, thread::current,
};


#[macro_use] extern crate prettytable;

fn main() {
    let now = Instant::now();

    let data = parse_input(&fs::read_to_string("input.txt").expect("Error reading file"));
    let elapsed_parse = now.elapsed();

    let now_1 = Instant::now();
    let answer_1 = part1(&data);
    let elapsed_1 = now_1.elapsed();

    let now_2 = Instant::now();
    let answer_2 = part2(&data);
    let elapsed_2 = now_2.elapsed();

    let elapsed_total = elapsed_parse + elapsed_1 + elapsed_2;

    ptable!(["Task", "Run Time (μs)", "Answer"],
        ["Total", elapsed_total.as_micros(), "-"],
        ["Parse", elapsed_parse.as_micros(), "-"],
        ["Part 1", elapsed_1.as_micros(), answer_1],
        ["Part 2", elapsed_2.as_micros(), answer_2]);
}

fn part1(data: &Vec<Vec<MapKey>>) -> u64 {
    let start_x = data.iter()
        .position(|line| line.contains(&MapKey::Start))
        .unwrap();
    let start_y = data[start_x as usize]
        .iter()
        .position(|start| *start == MapKey::Start)
        .unwrap();
    let mut previous_bread_crumb = BreadCrumb {
        position: (start_y, start_x),
        pipe: MapKey::Start,
        step_count: 0
    };
    let mut current_bread_crumb = BreadCrumb {
        position: (start_y, start_x),
        pipe: MapKey::Start,
        step_count: 0
    };

    let new_pos = begining(&current_bread_crumb, &data);
    current_bread_crumb.position = new_pos;
    current_bread_crumb.pipe = data[new_pos.0][new_pos.1];
    current_bread_crumb.step_count += 1;


    while current_bread_crumb.pipe != MapKey::Start {
        let pipe = current_bread_crumb.pipe;
        let current_pos = current_bread_crumb.position;
        let previous_pos = previous_bread_crumb.position;

        let new_pos = next_position(pipe, current_pos, previous_pos);

        previous_bread_crumb = current_bread_crumb.clone();
        current_bread_crumb.position = new_pos;
        current_bread_crumb.pipe = data[new_pos.0][new_pos.1];
        current_bread_crumb.step_count += 1;
    }
    let answer = current_bread_crumb.step_count / 2;
    answer
}

fn part2(data: &Vec<Vec<MapKey>>) -> i64 {
    0
}
fn parse_input(input: &str) -> Vec<Vec<MapKey>> {
    input
        .lines()
        .map(|line| line
            .chars().map(|character| MapKey::char_to_key(character))
            .collect::<Vec<MapKey>>())
        .collect::<Vec<Vec<MapKey>>>()
}

fn begining(bread_crumb: &BreadCrumb, data: &Vec<Vec<MapKey>>) -> (usize, usize) {
    let (x, y) = bread_crumb.position;
    let mut north = MapKey::Start;
    let mut south = MapKey::Start;
    let mut east = MapKey::Start;
    let mut west = MapKey::Start;

    if y > 0 {
        north = data[y-1][x];
    } else {
        north = MapKey::Start
    }
    if y < (data.len() - 1) {
        south = data[y+1][x];
    } else {
        south = MapKey::Start;
    }
    if x < (data.len() - 1) {
        east = data[y][x+1];
    } else {
        east = MapKey::Start;
    }
    if x > 0 {
        west = data[y][x-1];
    } else {
        west = MapKey::Start;
    }
    if north == MapKey::NorthSouth || north == MapKey::SouthEast || north == MapKey::SouthWest {
        (y - 1, x)
    } else if south == MapKey::NorthSouth || south == MapKey::NorthEast || south == MapKey::NorthWest {
        (y + 1, x)
    } else if east == MapKey::EastWest || east == MapKey::NorthWest || east == MapKey::SouthWest {
        (y, x + 1)
    } else if west == MapKey::EastWest || west == MapKey::NorthEast || west == MapKey::SouthEast{
        (y, x - 1)
    } else {
        panic!("Unable to find next after start");
    }
}

fn next_position(pipe: MapKey, current: (usize, usize), previous: (usize, usize)) -> (usize, usize){
    let mut current_y = current.0 as i64;
    let mut current_x = current.1 as i64;
    let previous_y = previous.0 as i64;
    let previous_x = previous.1 as i64;

    match pipe {
        MapKey::NorthSouth => {
            let inverted = current_y - previous_y;
            current_y = current_y + inverted;
        },
        MapKey::EastWest =>  {
            let inverted = current_x - previous_x;
            current_x = current_x + inverted;
        },
        MapKey::NorthEast => {
            if current_y == previous_y {
                current_y -= 1;
            } else {
                current_x += 1;
            }
        },
        MapKey::NorthWest => {
            if current_y == previous_y {
                current_y -= 1;
            } else {
                current_x -= 1;
            }
        },
        MapKey::SouthWest => {
            if current_y == previous_y {
                current_y += 1;
            } else {
                current_x -= 1;
            }
        },
        MapKey::SouthEast => {
            if current_y == previous_y {
                current_y += 1;
            } else {
                current_x += 1;
            }
        },
        MapKey::Start => {
            panic!("This shouldn't happen");
        },
        MapKey::Ground => {
            panic!("Ewww, you touched grass!");
        }
    }
    (current_y as usize, current_x as usize)
} 

#[derive(Clone)]
struct BreadCrumb {
    position: (usize, usize),
    pipe: MapKey,
    step_count: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MapKey {
    NorthSouth,     // |
    EastWest,       // -
    NorthEast,      // L
    NorthWest,      // J
    SouthWest,      // 7
    SouthEast,      // F
    Ground,         // .
    Start,          // S
}

impl MapKey {
    fn char_to_key(character: char) -> MapKey {
        match character {
            '|' => MapKey::NorthSouth,
            '-' => MapKey::EastWest,
            'L' => MapKey::NorthEast,
            'J' => MapKey::NorthWest,
            '7' => MapKey::SouthWest,
            'F' => MapKey::SouthEast,
            '.' => MapKey::Ground,
            'S' => MapKey::Start,
            _ => panic!("Are you sure you have the right map?"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";

        let input2 = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        
        let data = parse_input(&input);
        let answer = part1(&data);
        assert_eq!(answer, 4);

        let data = parse_input(&input2);
        let answer = part1(&data);
        assert_eq!(answer, 8);
    }
}