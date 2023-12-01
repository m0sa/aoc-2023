fn solve<F>(input: &str, parse_digit: F) -> i128
where
    F: Fn(usize, char, &str) -> Option<u32>,
{
    let numbers = input.split("\n").map(|line| {
        let digits: Vec<i128> = line
            .char_indices()
            .filter_map(|(i, c)| parse_digit(i, c, line))
            .map(i128::from)
            .collect();

        return digits.first().unwrap() * 10 + digits.last().unwrap();
    });

    return numbers.sum();
}

pub fn part1(input: &str) -> i128 {
    return solve(input, |_, c, _| c.to_digit(10));
}

use regex::Regex;
pub fn part2(input: &str) -> i128 {
    let numb_re = Regex::new(r"one|two|three|four|five|six|seven|eight|nine").unwrap();
    return solve(input, |i, c, line| {
        c.to_digit(10).or_else(|| {
            numb_re
                .find_at(line, i)
                .and_then(|m| match (m.start(), m.as_str()) {
                    (pos, _) if pos != i => None,
                    (_, "one") => Some(1),
                    (_, "two") => Some(2),
                    (_, "three") => Some(3),
                    (_, "four") => Some(4),
                    (_, "five") => Some(5),
                    (_, "six") => Some(6),
                    (_, "seven") => Some(7),
                    (_, "eight") => Some(8),
                    (_, "nine") => Some(9),
                    _ => None,
                })
        })
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn part1_example() {
        let example = r#"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
            "#
        .trim();
        let result = part1(example);
        assert_eq!(result, 142)
    }

    #[test]
    fn part1_result() {
        let input = utils::resource("src/day01.txt");
        let result = part1(&input);
        assert_eq!(result, 55621);
    }

    #[test]
    fn part2_example() {
        let example = r#"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#
            .trim();
        let result = part2(example);
        assert_eq!(result, 281)
    }

    #[test]
    fn part2_gotcha() {
        let result = part2("oneight");
        assert_eq!(result, 18);
    }

    #[test]
    fn part2_result() {
        let input = utils::resource("src/day01.txt");
        let result = part2(&input);
        assert_ne!(result, 53587); // that's the result we get if we don't respect overlapping matches, see gotcha
        assert_eq!(result, 53592);
    }
}
