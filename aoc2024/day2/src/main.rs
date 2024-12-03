use std::{fs, time::Instant};
extern crate humantime;
use humantime::format_duration;

extern crate nom;
use nom::{
    bytes::complete::tag, character::complete::i64, multi::separated_list1, Finish, IResult,
};

#[macro_use]
extern crate prettytable;

fn main() {
    let input = fs::read_to_string("day2/input.txt").expect("Error reading file");

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
    let data = parse_rows(&input);
    let mut safe_reports_count = 0;
    for x in 0..data.len() {
        if is_safe(&data[x]) {
            safe_reports_count += 1;
        }
    }
    safe_reports_count
}

fn part2(input: &str) -> isize {
    let data = parse_rows(&input);
    let mut safe_reports_count = 0;
    for x in 0..data.len() {
        if is_safe(&data[x]) {
            safe_reports_count += 1;
        } else {
            for y in 0..data[x].len() {
                let mut damp = data[x].clone();
                let _ = damp.remove(y);
                if is_safe(&damp) {
                    safe_reports_count += 1;
                    break;
                }
            }
        }
        // if is_safe_dampened(&data[x]) {
        //     safe_reports_count += 1;
        // }
    }
    safe_reports_count
}

fn is_increasing(report: &Vec<i64>) -> bool {
    for x in 0..(report.len() - 1) {
        let opt1 = report[x];
        let opt2 = report[x + 1];
        let inc = opt1 < opt2;
        if !inc {
            return false;
        }
        let diff = opt2 - opt1;
        let gt3 = diff > 3;
        if gt3 {
            return false;
        }
    }
    true
}

fn is_decreasing(report: &Vec<i64>) -> bool {
    for x in 0..(report.len() - 1) {
        let opt1 = report[x];
        let opt2 = report[x + 1];
        let dec = opt1 > opt2;
        if !dec {
            return false;
        }
        let diff = opt1 - opt2;
        let gt3 = diff > 3;
        if gt3 {
            return false;
        }
    }
    true
}

fn is_safe(report: &Vec<i64>) -> bool {
    if is_increasing(&report) || is_decreasing(&report) {
        return true;
    } else {
        return false;
    }
}

// fn is_increasing_dampened(report: &Vec<i64>) -> bool {
//     for x in 0..(report.len() - 1) {
//         let opt1 = report[x];
//         let opt2 = report[x + 1];
//         let inc = opt1 < opt2;
//         if !inc {
//             return false;
//         }
//         let diff = opt2 - opt1;
//         let gt3 = diff > 3;
//         if gt3 {
//             return false;
//         }
//     }
//     true
// }

// fn is_decreasing_dampened(report: &Vec<i64>) -> bool {
//     let mut dampening_count = 0;
//     for x in 0..(report.len() - 1) {
//         let opt1 = report[x];
//         let opt2 = report[x + 1];
//         let dec = opt1 > opt2;
//         if !dec {
//             dampening_count += 1;
//         }
//         let diff = opt1 - opt2;
//         let gt3 = diff > 3;
//         if gt3 {
//             return false;
//         }
//     }
//     if dampening_count <= 1 {
//         return true;
//     } else {
//         return false;
//     }
// }

// fn is_safe_dampened(report: &Vec<i64>) -> bool {
//     println!("{:?}", report);
//     if is_increasing_dampened(&report) || is_decreasing_dampened(&report) {
//         return true;
//     } else {
//         return false;
//     }
// }

fn parse_rows(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .into_iter()
        .map(|x| parse_col(&x).finish().unwrap().1)
        .collect()
}

fn parse_col(input: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(tag(" "), i64)(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_row() {
        let input = fs::read_to_string("example.txt").expect("Error reading file");
        let data = parse_rows(&input);

        let expected = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];
        assert_eq!(data, expected);
    }

    #[test]
    fn test_increasing() {
        let input = fs::read_to_string("example.txt").expect("Error reading file");
        let rows = parse_rows(&input);

        assert!(!is_increasing(&rows[0]));
        assert!(!is_increasing(&rows[1]));
        assert!(!is_increasing(&rows[2]));
        assert!(!is_increasing(&rows[3]));
        assert!(!is_increasing(&rows[4]));
        assert!(is_increasing(&rows[5]));
    }

    #[test]
    fn test_decreasing() {
        let input = fs::read_to_string("example.txt").expect("Error reading file");
        let rows = parse_rows(&input);

        assert!(is_decreasing(&rows[0]));
        assert!(!is_decreasing(&rows[1]));
        assert!(!is_decreasing(&rows[2]));
        assert!(!is_decreasing(&rows[3]));
        assert!(!is_decreasing(&rows[4]));
        assert!(!is_decreasing(&rows[5]));
    }

    #[test]
    fn test_is_safe() {
        let input = fs::read_to_string("example.txt").expect("Error reading files");
        let rows = parse_rows(&input);
        let mut safe_count = 0;

        for x in 0..rows.len() {
            if is_safe(&rows[x]) {
                safe_count += 1;
            }
        }
        assert_eq!(2, safe_count);
    }

    // #[test]
    // fn test_is_safe_dampened1() {
    //     let rows = vec![75, 77, 80, 78, 80, 83, 84, 87];
    //     assert!(is_safe_dampened(&rows));
    // }

    // #[test]
    // fn test_is_safe_dampened() {
    //     let input = fs::read_to_string("example.txt").expect("Error reading files");
    //     let rows = parse_rows(&input);
    //     let mut safe_count = 0;

    //     for x in 0..rows.len() {
    //         if is_safe_dampened(&rows[x]) {
    //             safe_count += 1;
    //         }
    //     }
    //     assert_eq!(4, safe_count);
    // }
}
