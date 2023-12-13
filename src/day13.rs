type Int = usize;
type Mirror = (Vec<String>, Vec<String>);

fn find_reflection(input: &Vec<String>, skip: Option<Int>) -> Option<Int> {
    for candidate in 1..input.len() {
        let mut low: isize = candidate as isize - 1;
        let mut high = candidate;
        let mut all_matches = low >= 0;
        while low >= 0 && high < input.len() && all_matches {
            all_matches = all_matches && input[low as usize] == input[high];
            low = low - 1;
            high = high + 1;
        }
        if all_matches {
            let result = Some(candidate);
            if result != skip {
                return result;
            }
        }
    }
    return None;
}

fn parse_input(input: &str) -> Vec<Mirror> {
    let mut mirrors = Vec::<Mirror>::new();
    let mut lines = Vec::<String>::new();
    let mut lines_cnt = 0;
    let mut line_len = 0;
    for line in input.split('\n').map(|x| x.trim()) {
        if line.len() > 0 {
            lines.push(line.to_string());
            line_len = line.len();
            lines_cnt = lines_cnt + 1;
            continue;
        }
        if lines_cnt == 0 {
            continue;
        }
        // we have a bunch of lines queue up and we've reched an empty line
        let mut columns = Vec::<String>::new();
        for c in 0..line_len {
            let mut column_str = "".to_string();
            for ln in lines.iter() {
                column_str.push(ln.chars().nth(c).unwrap());
            }
            columns.push(column_str.to_string());
        }

        mirrors.push((lines.clone(), columns));
        lines.clear();
        line_len = 0;
        lines_cnt = 0;
    }
    return mirrors;
}

fn flip_at(input: &String, index: usize) -> String {
    input
        .as_bytes()
        .iter()
        .enumerate()
        .map(|(i, &ch)| {
            if i == index {
                match ch {
                    b'.' => '#',
                    b'#' => '.',
                    x => panic!("unexpected char: '{}'", x as char),
                }
            } else {
                ch as char
            }
        })
        .collect()
}

fn replace_at(input: &Vec<String>, with: &String, at: usize) -> Vec<String> {
    input
        .iter()
        .enumerate()
        .map(|(i, str)| {
            if i == at {
                with.to_string()
            } else {
                str.to_string()
            }
        })
        .collect()
}

fn part1(input: &str) -> Int {
    parse_input(input)
        .iter()
        .map(|(lines, columns)| {
            find_reflection(&columns, None)
                .or(find_reflection(&lines, None).map(|h| h * 100))
                .expect("expecting exactly one reflection")
        })
        .sum()
}

fn with_smudge_fixed(mirror: &Mirror) -> Int {
    let (lines, columns) = mirror;
    let except_horizontal = find_reflection(&lines, None);
    let except_vertical = find_reflection(&columns, None);
    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            let smudged_line = flip_at(&lines[y], x);
            let smudged_line_fixed = replace_at(&lines, &smudged_line, y);
            let found_horizontal = find_reflection(&smudged_line_fixed, except_horizontal);
            if found_horizontal.is_some() {
                return found_horizontal.unwrap() * 100;
            }

            let smudged_column = flip_at(&columns[x], y);
            let smudged_column_fixed = replace_at(&columns, &smudged_column, x);
            let found_vertical = find_reflection(&smudged_column_fixed, except_vertical);
            if found_vertical.is_some() {
                return found_vertical.unwrap();
            }
        }
    }

    panic!("expecting exactly one smudge");
}

fn part2(input: &str) -> Int {
    parse_input(input).iter().map(with_smudge_fixed).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;
    const EXAMPLE: &str = r#"
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
"#;
    #[test]
    fn part1_example() {
        let result = part1(EXAMPLE);
        assert_eq!(405, result);
    }

    #[test]
    fn part1_result() {
        let result = part1(&resource("src/day13.txt"));
        assert_eq!(29846, result);
    }

    #[test]
    fn part2_example() {
        let result = part2(EXAMPLE);
        assert_eq!(400, result);
    }

    #[test]
    fn part2_result() {
        let result = part2(&resource("src/day13.txt"));
        assert_eq!(25401, result);
    }
}
