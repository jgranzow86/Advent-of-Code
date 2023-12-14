use std::{
    fs,
    time::Instant,
};


#[macro_use] extern crate prettytable;

fn main() {
    let now = Instant::now();

    let input = fs::read_to_string("input.txt")
        .expect("Error reading file");
    let data = parse_input(&input);
    let expansions = find_expansions(&data);
    let galaxies = find_galaxies(&data);
    let elapsed_parse = now.elapsed();

    let now_1 = Instant::now();
    let answer_1 = part1(&galaxies, &expansions, 1);
    let elapsed_1 = now_1.elapsed();

    let now_2 = Instant::now();
    let answer_2 = part2(&galaxies, &expansions, 999_999);
    let elapsed_2 = now_2.elapsed();

    let elapsed_total = elapsed_parse + elapsed_1 + elapsed_2;

    ptable!(["Task", "Run Time (μs)", "Answer"],
        ["Total", elapsed_total.as_micros(), "-"],
        ["Parse", elapsed_parse.as_micros(), "-"],
        ["Part 1", elapsed_1.as_micros(), answer_1],
        ["Part 2", elapsed_2.as_micros(), answer_2]);
}

fn part1(galaxies: &Vec<Galaxy>, expansions: &(Vec<usize>, Vec<usize>), expansion_rate: i64) -> u64 {
    let mut answer = Vec::new();

    let galaxies = expand(&galaxies, &expansions, expansion_rate);

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

fn part2(galaxies: &Vec<Galaxy>, expansions: &(Vec<usize>, Vec<usize>), expansion_rate: i64) -> u64 {
    let mut answer = Vec::new();

    let galaxies = expand(&galaxies, &expansions, expansion_rate);

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

fn find_expansions(data: &Vec<Vec<Space>>) -> (Vec<usize>, Vec<usize>) {
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

    (rows_to_expand, columns_to_expand)
}

fn expand(galaxies: &Vec<Galaxy>, expansions: &(Vec<usize>, Vec<usize>), expansion_rate: i64) -> Vec<Galaxy> {
    let mut galaxies = galaxies.clone();
    let mut rows_to_expand = expansions.0.clone();
    rows_to_expand.reverse();
    let mut columns_to_expand = expansions.1.clone();
    columns_to_expand.reverse();

    for col in columns_to_expand {
        let mut idx = 0;
        while idx < galaxies.len() {
            if galaxies[idx].x > col as i64 {
                galaxies[idx].x += expansion_rate;
            }
            idx += 1;
        }
    }

    for row in rows_to_expand {
        let mut idx = 0;
        while idx < galaxies.len() {
            if galaxies[idx].y > row as i64 {
                galaxies[idx].y += expansion_rate;
            }
            idx += 1;
        }
    }
    galaxies
}

fn find_galaxies(data: &Vec<Vec<Space>>) -> Vec<Galaxy> {
    let mut galaxies = Vec::new();
    {
        let mut id = 1;
        let mut y = 0;
        while y < data.len() {
            let mut x = 0;
            while x < data[y].len() {
                if data[y][x] == Space::Gal { 
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
        let expansions = find_expansions(&data);
        let galaxies = find_galaxies(&data);
        let answer = part1(&galaxies, &expansions, 1);
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
        let galaxies = find_galaxies(&data);
        let expansions = find_expansions(&data);
        let answer = part2(&galaxies, &expansions, 9);
        assert_eq!(answer, 1030);
        let answer = part2(&galaxies, &expansions, 99);
        assert_eq!(answer, 8410);
    }

    #[test]
    fn test_bed() {
        let start = Instant::now();
        for _ in 0..=1_000_000 {
            
        }
        let elapsed = start.elapsed();
        println!("Run time: {}µs", elapsed.as_micros());
        assert!(elapsed.as_micros() < 10_000);
    }
}