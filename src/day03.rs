use regex::Regex;
use std::collections::HashMap;
use crate::utils::Point2D;

fn solve(input: &str) -> (i32, i32) {
    let mut line_number = 0;
    let mut symbols = HashMap::<Point2D, char>::new();
    let number_re = Regex::new(r"[0-9]+").unwrap();
    let symbol_re = Regex::new(r"[^.0-9]").unwrap(); // TODO are numbers symbols too?
    let lines: Vec<&str>  = input.split('\n').map(|ln| ln).collect();
    for line in (&lines).into_iter() {
        for symbol_match in symbol_re.find_iter(line) {
            let point =  Point2D { x: symbol_match.start() as i32, y: line_number };
            symbols.insert(point, symbol_match.as_str().chars().nth(0).unwrap());
        }
        line_number += 1;
    }

    let mut sum_all = 0;
    line_number = 0;
    let mut asterisks_adj = HashMap::<Point2D, Vec<i32>>::new();
    for line in (&lines).into_iter() {
        for n in number_re.find_iter(line) {
            let mut border = Vec::<Point2D>::new();
            let col_prev = n.start() as i32 - 1;
            let col_next = n.end() as i32;
            let line_up = line_number - 1;
            let line_dn = line_number + 1;
            let number = n.as_str().parse::<i32>().expect("expected integer");
            for x in n.range() {
                border.push((x as i32, line_up).into());
                border.push((x as i32, line_dn).into());
            }
            for y in line_up..line_dn+1 {
                border.push((col_prev, y).into());
                border.push((col_next, y).into());
            }
            let symbols: Vec<(char, Point2D)> = border.into_iter().filter_map(|b| symbols.get(&b).map(|c| (*c, b))).collect();
            if symbols.len() > 0 {
                sum_all += number;
            }
            for asterisk_pos in symbols.into_iter().filter_map(|x| match x.0 { '*' => Some(x.1), _ => None }) {
                let existing = asterisks_adj.get_mut(&asterisk_pos);
                if existing.is_some() {
                    existing.unwrap().push(number);
                } else {
                    asterisks_adj.insert(asterisk_pos, vec![number]);
                }                
            }
        }
        line_number += 1;
    }

    let gear_ratios = asterisks_adj.into_iter()
        .filter(|(_, numbers)| numbers.len() > 1)
        .map(|(_,numbers)| numbers
            .into_iter()
            .fold(1, |agg,next| agg * next));
    return (sum_all, gear_ratios.sum());
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    const EXAMPLE: &str = r#"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
        "#;
    #[test]
    fn part1_example() {
        let result = solve(EXAMPLE.trim()).0;

        assert_eq!(result, 4361);
    }

    #[test]
    fn part1_result() {
        let input = utils::resource("src/day03.txt");
        let result = solve(&input).0;

        assert_eq!(result, 540212);
    }

    #[test]
    fn part2_example() {
        let result = solve(EXAMPLE.trim()).1;

        assert_eq!(result, 467835);
    }

    #[test]
    fn part2_result() {
        let input = utils::resource("src/day03.txt");
        let result = solve(&input).1;

        assert_eq!(result, 87605697);
    }
}
