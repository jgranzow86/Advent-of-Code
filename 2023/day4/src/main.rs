use std::collections::HashSet;
// use std::iter::FromIterator;

use nom::{IResult,
    sequence::tuple,
    bytes::complete::tag,
    character::complete,
    character::complete::{space1, alpha1, digit1},
    branch::alt, Finish,
};

fn main() {
    let input = include_str!("input.txt");


    let part1_answer = part1(input);
    println!("Part 1: {part1_answer}");
}

fn part1(input: &str) -> u64 {
    let mut total_score = 0;
    let cards: Vec<ScratchCard> = input
        .lines()
        .into_iter()
        .map(|line| ScratchCard::new_from_str(line))
        .collect();

    for card in cards {
        let mut win_count = 0;
        for drawn_number in card.drawn_numbers {
            if card.winning_numbers.contains(&drawn_number) {
                win_count += 1;
            }
        }
        if win_count != 0 {
            total_score += 2u64.pow((win_count - 1) as u32);
        }
    }
    total_score
}

struct ScratchCard {
    id: u64,
    winning_numbers: Vec<u64>,
    drawn_numbers: Vec<u64>,
}

impl ScratchCard {
    fn new_from_str(input: &str) -> ScratchCard {
        let (input, id) = parse_card_id(input).finish().unwrap();
        let (input, winning_numbers) = parse_winning_numbers(input).finish().unwrap();
        let (_, drawn_numbers) = parse_number_list(input).finish().unwrap();
        let scratch_card = ScratchCard{id, winning_numbers, drawn_numbers};
        scratch_card
    }
}

fn parse_colon(input: &str) -> IResult<&str, ()> {
    let (input, _) = tag(":")(input)?;
    Ok((input, ()))
}

fn parse_card_id(input: &str) -> IResult<&str, u64> {
    let (input, (_, _, id, _, _)) = tuple((alpha1, space1, complete::u64, parse_colon, space1))(input)?;
    Ok((input, id))
}
fn parse_pipe(input: &str) -> IResult<&str, &str> {
    let (input, pipe) = tag("|")(input)?;
    Ok((input, pipe))
}

fn parse_winning_numbers(input: &str) -> IResult<&str, Vec<u64>> {
    let mut winning_numbers = Vec::new();
    let mut input = input;
    let mut winning_number: &str;
    loop {
        (input, (winning_number)) = alt((digit1, space1, parse_pipe))(input)?;
        if nom::character::is_digit(winning_number.chars().nth(0).unwrap() as u8) {
            winning_numbers.push(winning_number.parse::<u64>().unwrap());
        } else if !nom::character::is_space(winning_number.chars().nth(0).unwrap() as u8) {
            break;
        }
    }
    Ok((input, winning_numbers))
}

fn parse_number_list(input: &str) -> IResult<&str, Vec::<u64>> {
    let mut number_list = Vec::new();
    let mut input = input;
    let mut number: u64;
    loop {
        (input, (_, number)) = tuple((space1, complete::u64))(input)?;
        number_list.push(number);
        if input.is_empty() {
            break;
        }
    }
    Ok((input, number_list))
}


#[cfg(test)]
mod tests {
    use nom::Finish;

    use super::*;
    #[test]
    fn test_part_1() {
        let sample = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let sum = part1(&sample);
        assert_eq!(sum, 13);
    }

    #[test]
    fn test_parse_card_id() {
        let sample = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card_id = parse_card_id(sample).finish();
        let test = card_id.unwrap();
        assert_eq!(test, ("41 48 83 86 17 | 83 86  6 31 17  9 48 53", 1));

        let sample = "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19";
        let card_id = parse_card_id(sample).finish().unwrap();
        assert_eq!(card_id, ("13 32 20 16 61 | 61 30 68 82 17 32 24 19", 2));

        let sample = "Card 420:  1 21 53 59 44 | 69 82 63 72 16 21 14  1";
        let card_id = parse_card_id(sample).finish().unwrap();
        assert_eq!(card_id, ("1 21 53 59 44 | 69 82 63 72 16 21 14  1", 420));
    }

    #[test]
    fn test_parse_winning_numbers() {
        let sample = "1 21 53 59 44 | 69 82 63 72 16 21 14  1";
        let expected_results = (" 69 82 63 72 16 21 14  1", vec![1, 21, 53,59, 44]);
        let results = parse_winning_numbers(sample).finish().unwrap();
        assert_eq!(results, expected_results);
    }

    #[test]
    fn test_parse_number_list() {
        let sample = " 69 82 63 72 16 21 14  1";
        let expected_results = ("", vec![69, 82, 63, 72, 16, 21, 14, 1]);
        let results = parse_number_list(sample).finish().unwrap();
        assert_eq!(results, expected_results);
    }
}