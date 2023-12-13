use core::panic;
use std::{
    fs,
    time::Instant,
};
use prettytable::{Table, Row, Cell};


#[macro_use] extern crate prettytable;

fn main() {
    let now = Instant::now();

    let input = fs::read_to_string("input.txt")
        .expect("Error reading file");
    let data = parse_input(&input);
    let expanded_data = expand(&data, 1);
    let galaxies = find_galaxies(&expanded_data);
    let elapsed_parse = now.elapsed();

    let now_1 = Instant::now();
    let answer_1 = part1(&galaxies);
    let elapsed_1 = now_1.elapsed();

    let now_2 = Instant::now();
    // let answer_2 = part2(&data);
    let answer_2 = 0;
    let elapsed_2 = now_2.elapsed();

    let elapsed_total = elapsed_parse + elapsed_1 + elapsed_2;

    ptable!(["Task", "Run Time (μs)", "Answer"],
        ["Total", elapsed_total.as_micros(), "-"],
        ["Parse", elapsed_parse.as_micros(), "-"],
        ["Part 1", elapsed_1.as_micros(), answer_1],
        ["Part 2", elapsed_2.as_micros(), answer_2]);
}

fn part1(galaxies: &Vec<Galaxy>) -> u64 {
    let mut answer = Vec::new();

    {
        let mut gal = 0;
        while gal < galaxies.len() {
            let mut galx = galaxies[gal].id as usize;
            while galx < galaxies.len() {
                answer.push(galaxies[gal]
                    .calculate_neighbors_distance(&galaxies[galx]));
                galx += 1;
            }
            gal += 1;
        }
    }
    answer.iter().sum()
}

fn part2(data: &Vec<Vec<char>>) -> i64 {
    0
}

#[derive(PartialEq, Eq, Clone, Debug)]
enum Space {
    Gal,
    Nothing,
}

fn parse_input(input: &str) -> Vec<Vec<Space>> {
    // let mut y_lines = Vec::new();
    let data = input
        .lines()
        .map(|line| line
            .chars()
            .map(|character| {
                if character == '.' {
                    Space::Nothing
                } else {
                    Space::Gal
                }})
            .collect())
        .collect::<Vec<Vec<Space>>>();

    data
}

fn expand(data: &Vec<Vec<Space>>, count: u64) -> Vec<Vec<Space>> {
    let mut expanded_data = data.clone();
    let mut columns_to_expand = Vec::new();
    let mut rows_to_expand = Vec::new();

    // Calculate addtional columns
    {
        let mut x = 0;
        while x < data[0].len() {
            let mut y = 0;
            while y < data.len() {
                if data[y][x] == Space::Gal {
                    break;
                } else if y == (data.len() - 1) {
                    columns_to_expand.push(x);
                }
                y += 1;
            }
            x += 1;
        }
    }

    // Calculate additional rows
    {
        let mut y = 0;
        while y < data.len() {
            let mut x = 0;
            while x < data[0].len() {
                if data[y][x] == Space::Gal {
                    break;
                } else if x == (data[0].len() - 1) {
                    rows_to_expand.push(y);
                }
                x += 1;
            }
            y += 1;
        }
    }

    rows_to_expand.reverse();
    for each in rows_to_expand {
        expanded_data.insert(each, vec![Space::Nothing; data[0].len()]);
    }

    columns_to_expand.reverse();
    for each in columns_to_expand {
        let mut y = 0;
        while y < expanded_data.len() {
            expanded_data[y].insert(each, Space::Nothing);
            y += 1;
        }
    }

    expanded_data
}

fn find_galaxies(expanded_data: &Vec<Vec<Space>>) -> Vec<Galaxy> {
    let mut galaxies = Vec::new();
    {
        let mut id = 1;
        let mut y = 0;
        while y < expanded_data.len() {
            let mut x = 0;
            while x < expanded_data[y].len() {
                if expanded_data[y][x] == Space::Gal { 
                    galaxies.push(Galaxy { id, x: x as i64, y: y as i64 });
                    id += 1;
                }
                x += 1;
            }
            y += 1;
        }
    }
    galaxies
}

#[derive(Clone, Debug)]
struct Galaxy {
    id: u64,
    x: i64,
    y: i64,
}

impl Galaxy {
    fn calculate_neighbors_distance(&self, neighbor: &Galaxy) -> u64 {
        let x_offset = (self.x - neighbor.x).abs();
        let y_offset = (self.y - neighbor.y).abs();

        (x_offset + y_offset) as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        
        let data = parse_input(&input);
        let expanded_data = expand(&data, 1);
        let galaxies = find_galaxies(&expanded_data);
        let answer = part1(&galaxies);
        assert_eq!(answer, 374);
    }

    #[test]
    fn test_part_2() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        
        let data = parse_input(&input);
        let expanded_data = expand(&data, 1);
        let galaxies = find_galaxies(&expanded_data);
        let answer = part1(&galaxies);
        assert_eq!(answer, 8410);
    }

    #[test]
    fn test_expansion() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        let expexcted_input = "....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......";

        let data = parse_input(&input);
        let expected_data = parse_input(&expexcted_input);
        let expanded_data = expand(&data, 1);

        assert_eq!(expected_data, expanded_data);
    }
}