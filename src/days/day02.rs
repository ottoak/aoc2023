use nom::branch::alt;
use nom::{
    bytes::complete::tag, character::complete::space1, multi::separated_list1,
    sequence::separated_pair, IResult,
};

use crate::utils::read_lines;

fn ball_parser(s: &str) -> IResult<&str, (u32, &str)> {
    separated_pair(
        nom::character::complete::u32,
        space1,
        alt((tag("red"), tag("blue"), tag("green"))),
    )(s)
}

fn balls_parser(s: &str) -> IResult<&str, Vec<(u32, &str)>> {
    separated_list1(alt((tag(", "), tag("; "))), ball_parser)(s)
}

#[derive(Default)]
struct Set {
    r: u32,
    g: u32,
    b: u32,
}

impl Set {
    fn from_balls(balls: Vec<(u32, &str)>) -> Self {
        let mut s = Set { r: 0, g: 0, b: 0 };

        for (n, col) in balls {
            match col {
                "red" => {
                    if s.r < n {
                        s.r = n
                    }
                }
                "green" => {
                    if s.g < n {
                        s.g = n
                    }
                }
                "blue" => {
                    if s.b < n {
                        s.b = n
                    }
                }
                _ => (),
            }
        }

        s
    }
}

fn get_id_if_valid(g: String) -> u32 {
    let (id, games) = g.split_once(": ").unwrap();
    let id = id.replace("Game ", "").parse::<u32>().unwrap();

    let (_, parsed_balls) = balls_parser(games).unwrap();

    let s = Set::from_balls(parsed_balls);

    if s.r <= 12 && s.g <= 13 && s.b <= 14 {
        id
    } else {
        0
    }
}

fn get_valid_score(g: String) -> u32 {
    let (_, games) = g.split_once(": ").unwrap();

    let (_, parsed_balls) = balls_parser(games).unwrap();

    let s = Set::from_balls(parsed_balls);

    s.r * s.g * s.b
}

fn part1(input: Vec<String>) -> u32 {
    input
        .into_iter()
        .map(get_id_if_valid)
        .collect::<Vec<u32>>()
        .iter()
        .sum()
}

fn part2(input: Vec<String>) -> u32 {
    input
        .into_iter()
        .map(get_valid_score)
        .collect::<Vec<u32>>()
        .iter()
        .sum()
}

pub fn solve() -> (usize, usize) {
    let input = read_lines("./inputs/day2.txt");

    (part1(input.clone()) as usize, part2(input) as usize)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ball_parser() {
        let tests = [("3 blue", (3, "blue"), ""), ("56 red", (56, "red"), "")];

        for (input, expected_output, expected_remaining_input) in tests {
            let (remaining_input, output) = ball_parser(input).unwrap();
            assert_eq!(remaining_input, expected_remaining_input);
            assert_eq!(output, expected_output);
        }
    }

    #[test]
    fn test_balls_parser() {
        let tests = [
            (
                "3 blue, 4 red; 1 red, 2 green",
                vec![(3, "blue"), (4, "red"), (1, "red"), (2, "green")],
                "",
            ),
            (
                "8 green, 6 blue, 20 red; 5 blue, 4 red",
                vec![
                    (8, "green"),
                    (6, "blue"),
                    (20, "red"),
                    (5, "blue"),
                    (4, "red"),
                ],
                "",
            ),
        ];

        for (input, expected_output, expected_remaining_input) in tests {
            let (remaining_input, output) = balls_parser(input).unwrap();
            assert_eq!(remaining_input, expected_remaining_input);
            assert_eq!(output, expected_output);
        }
    }
    #[test]
    fn test_valid_game() {
        let tests = [
            ("Game 2: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 2),
            (
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
                0,
            ),
        ];

        for (input, expected_output) in tests {
            let valid = get_id_if_valid(input.to_string());
            assert_eq!(expected_output, valid);
        }
    }

    #[test]
    fn test_part1() {
        let sample = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let input = sample.lines().map(String::from).collect();

        assert_eq!(8, part1(input))
    }

    #[test]
    fn test_part2() {
        let sample = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let input = sample.lines().map(String::from).collect();

        assert_eq!(2286, part2(input))
    }
}
