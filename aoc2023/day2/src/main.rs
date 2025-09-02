use nom::{IResult,
    character::complete,
    character::complete::{space1, alpha1},
    bytes::complete::tag,
    sequence::tuple,
    branch::alt,
    Finish};

fn main() {
    let input = include_str!("input.txt");

   let part1_sum = part1(input);

    println!("Part 1 sum: {part1_sum}");

    let part2_sum = part2(input);

    println!("Part 2 sum: {part2_sum}");
}

fn part1(input: &str) -> u64 {
    let games = Game::collect_games(input);

    let mut sum = 0;
    for game in games {
        if 12 >= game.red && 13 >= game.green && 14 >= game.blue {
            sum += game.id;
        }
    }

    sum

}

fn part2(input: &str) -> u64 {
    let games = Game::collect_games(input);

    let mut sum = 0;
    for game in games {
        sum += game.red * game.green * game.blue;
    }

    sum

}

struct Game {
    id: u64,
    red: u64,
    green: u64,
    blue: u64,
}

impl Game {
    fn collect_games(input: &str) -> Vec<Self> {
        let mut games = Vec::new();

        for line in input.lines() {
            let (mut line, id) = game_id_parser(line).finish().unwrap();
            let (mut red, mut green, mut blue) = (0, 0, 0);
            
            loop {
                let (remaining_line, (count, color)) = count_color(line).finish().unwrap();
    
                match color {
                    "red" => {
                        if count > red {
                            red = count;
                        }
                    },
                    "green" => {
                        if count > green {
                            green = count;
                        }
                    },
                    "blue" => {
                        if count > blue {
                            blue = count;
                        }
                    },
                    _ => {
                        println!("You fucked up!");
                    }
                }
    
                if !remaining_line.is_empty() {
                    let (remaining_line, _) = consume_sep(remaining_line).finish().unwrap();
                    line = remaining_line;
                } else {
                    break;
                }
            }
            games.push(Game{id, red, green, blue});
        }
        games
    }
}

fn game_parser(input: &str) -> IResult<&str, ()> {
    let (input, _) = tag("Game ")(input)?;
    Ok((input, ()))    
}

fn game_junk_parser(input: &str) -> IResult<&str, ()> {
    let (input, _) = tag(": ")(input)?;
    Ok((input, ()))
}

fn game_id_parser(input: &str) -> IResult<&str, u64> {
    let (input, (_, id, _)) = tuple((game_parser, complete::u64, game_junk_parser))(input)?;
    Ok((input, id))
}

fn count_color(input: &str) -> IResult<&str, (u64, &str)> {
    let (input, (count, _, color)) = tuple((complete::u64, space1, alpha1))(input)?;
    Ok((input, (count, color)))
}

fn consume_sep(input: &str) -> IResult<&str, ()> {
    let comma_parse = tag(", ");
    let semi_parse = tag("; ");
    let (input, _) = alt((comma_parse, semi_parse))(input)?;
    Ok((input, ()))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let sample: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        
        assert_eq!(8, part1(sample));
    }

    #[test]
    fn test_part_2() {
        let sample: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        
        assert_eq!(2286, part2(sample));
    }

    #[test]
    fn test_getting_game_id() {
        let sample: &str = "Game 73: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let game_id = game_id_parser(&sample).finish().unwrap();
        assert_eq!(("6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 73), game_id);
    }

    #[test]
    fn test_getting_count_color() {
        let sample = "69 red";
        let count_color = count_color(sample).finish().unwrap();
        assert_eq!(("", (69, "red")), count_color);
    }

    #[test]
    fn test_consume_sep() {
        let sample = "; ";
        assert_eq!(("", ()), consume_sep(sample).finish().unwrap());
        let sample = ", ";
        assert_eq!(("", ()), consume_sep(sample).finish().unwrap());
    }
}