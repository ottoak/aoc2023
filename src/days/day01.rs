//use std::ptr::replace;

use crate::utils::read_lines;

fn combine_first_and_last(s: String) -> u32 {
	// Trim away any starting letters - the first and last digits (if any) will then
	// be the first and last characters in the string
	let trimmed = s.trim_matches(char::is_alphabetic);

	// Grab the first character of the string, and parse it to a digit
	let d1 = trimmed.chars().next().unwrap().to_digit(10).unwrap_or_default();
	// Grab the second character of the string, and parse it to a digit
	let d2 = trimmed.chars().last().unwrap().to_digit(10).unwrap_or_default();

	// Return two digit number with d1 in the tens spot & d2 in the ones spot
	d1*10 + d2
}

fn part1(input: Vec<String>) -> u32 {
	input
		.into_iter()
		.map(combine_first_and_last)
		.sum::<u32>()
}

// Solution inspired by:
// https://www.reddit.com/r/adventofcode/comments/1883ibu/2023_day_1_solutions/kblz9al/
fn part2(input: Vec<String>) -> u32 {
	// Clever way to replace string occurrences with digits, but ensuring tricky inputs like
	// "ab1twone" gets converted to "ab1twone" => "ab1t2o1e" => 11, instead of naively doing the
	// conversion "ab1twone" => "ab12ne" => 12
	let replacements = [
		["one", "o1e"],
		["two", "t2o"],
		["three", "th3ee"],
		["four", "fo4r"],
		["five", "fi5e"],
		["six", "s6x"],
		["seven", "se7en"],
		["eight", "ei8ht"],
		["nine", "n9ne"]
	];

	let mut new_lines= Vec::new();

	for line in input {
		let mut temp_str = line;
		for rep in replacements {
			temp_str = temp_str.replace(rep[0], rep[1]);
		}

		new_lines.push(temp_str);
	}

	part1(new_lines)
}

pub fn solve() -> (usize, usize) {
	let input = read_lines("./inputs/day4.txt");

	//(part1(input) as usize, part2(&input) as usize)
	(part1(input.clone()) as usize, part2(input) as usize)
}

#[cfg(test)]
mod tests {
	use super::{combine_first_and_last, part1, part2};

	#[test]
	fn test_sum_first_and_last() {
		let examples = vec![
			(38, "pqr3stu8vwx"),
			(15, "a1b2c3d4e5f"),
			(12, "1abc2"),
		];

		for (expected, input) in examples {
			assert_eq!(expected, combine_first_and_last(input.to_string()));
		}
	}

	#[test]
	fn test_part1() {
		let example = vec![
			"1abc2".to_string(),
			"pqr3stu8vwx".to_string(),
			"a1b2c3d4e5f".to_string(),
			"treb7uchet".to_string(),
		];

		let output = part1(example);

		assert_eq!(output, 142);
	}

	#[test]
	fn test_part2() {
		let example = vec![
			"two1nine".to_string(),
			"eightwothree".to_string(),
			"abcone2threexyz".to_string(),
			"xtwone3four".to_string(),
			"4nineeightseven2".to_string(),
			"zoneight234".to_string(),
			"7pqrstsixteen".to_string(),
		];

		let output = part2(example);

		assert_eq!(output, 281);
	}
}