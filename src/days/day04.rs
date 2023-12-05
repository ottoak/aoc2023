use nom::{
    bytes::complete::tag, character::complete::multispace1, combinator::map,
    multi::separated_list1, sequence::separated_pair, IResult,
};

use crate::utils::read_lines;

#[derive(Debug, Eq, PartialEq)]
pub struct Card {
    nums: Vec<u8>,
    winning_nums: Vec<u8>,
}

impl Card {
    fn vec_parser(s: &str) -> IResult<&str, Vec<u8>> {
        let trimmed = s.trim();
        separated_list1(multispace1, nom::character::complete::u8)(trimmed)
    }
    fn parse_card(input: &str) -> IResult<&str, Self> {
        let (_, card) = input.split_once(':').unwrap();
        let card = card.trim();

        let bar = tag(" | ");
        let parse_game = separated_pair(Card::vec_parser, bar, Card::vec_parser);

        map(parse_game, |(nums, winning_nums)| Card {
            nums,
            winning_nums,
        })(card)
    }
    fn get_score(self) -> usize {
        self.nums.into_iter().fold(0, |acc, n| {
            if self.winning_nums.contains(&n) {
                acc + 1
            } else {
                acc
            }
        })
    }
}

fn part1(input: Vec<String>) -> usize {
    let mut scores: Vec<usize> = vec![];

    for i in input {
        let (_, card) = Card::parse_card(i.as_str()).unwrap();

        let n_winning = card.get_score() as u32;

        if n_winning > 0 {
            scores.push(2usize.pow(n_winning - 1));
        } else {
            scores.push(0);
        }
    }

    scores.iter().sum()
}

fn part2(input: Vec<String>) -> usize {
    let mut scores: Vec<usize> = vec![];

    for i in input {
        let (_, card) = Card::parse_card(i.as_str()).unwrap();
        scores.push(card.get_score());
    }

    let mut num_cards = vec![1; scores.len()];

    for (i, val) in scores.iter().enumerate() {
        let start_card = i + 1;
        let end_card = start_card + val;
        let num_owned = *num_cards.get(i).unwrap();

        num_cards[start_card..end_card]
            .iter_mut()
            .for_each(|v| *v += num_owned);
    }

    num_cards.iter().sum()
}

pub fn solve() -> (usize, usize) {
    let input = read_lines("./inputs/day4.txt");

    let p1 = part1(input.clone());
    let p2 = part2(input);

    (p1, p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_vec() {
        let tests = [
            ("41 48 83 86 17", vec![41, 48, 83, 86, 17], ""),
            ("1  1 53 59 44", vec![1, 1, 53, 59, 44], ""),
        ];

        for (input, expected_output, expected_remaining_input) in tests {
            let (remaining_input, output) = Card::vec_parser(input).unwrap();
            assert_eq!(remaining_input, expected_remaining_input);
            assert_eq!(output, expected_output);
        }
    }
    #[test]
    fn test_parse_game() {
        let tests = [
            (
                "Card 1: 41 48 | 83 86",
                Card {
                    nums: vec![41, 48],
                    winning_nums: vec![83, 86],
                },
                "",
            ),
            (
                "Card 2:  6 10 | 11 12",
                Card {
                    nums: vec![6, 10],
                    winning_nums: vec![11, 12],
                },
                "",
            ),
        ];

        for (input, expected_output, expected_remaining_input) in tests {
            let (remaining_input, output) = Card::parse_card(input).unwrap();
            assert_eq!(remaining_input, expected_remaining_input);
            assert_eq!(output, expected_output);
        }
    }

    #[test]
    fn test_score() {
        let tests = [
            (
                4,
                Card {
                    nums: vec![41, 48, 83, 86, 17],
                    winning_nums: vec![83, 86, 6, 31, 17, 9, 48, 53],
                },
            ),
            (
                2,
                Card {
                    nums: vec![13, 32, 20, 16, 61],
                    winning_nums: vec![61, 30, 68, 82, 17, 32, 24, 19],
                },
            ),
        ];

        for (expected_winning, card) in tests {
            let actual_winning = card.get_score();
            assert_eq!(actual_winning, expected_winning);
        }
    }

    #[test]
    fn test_run() {
        let sample = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let input = sample.lines().map(String::from).collect();

        let score = part1(input);

        assert_eq!(13, score)
    }
}
