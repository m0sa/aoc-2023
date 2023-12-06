type Int = i128;

fn solve(times: &Vec<Int>, records: &Vec<Int>) -> Int {
    return times
        .iter()
        .zip(records.iter())
        .map(|(&time, &record)| {
            let options: Vec<Int> = (1..time)
                .into_iter()
                .filter_map(|hodl| {
                    let speed = hodl;
                    let remaining = time - speed;
                    let distance = remaining * speed;
                    return match distance {
                        x if x > record => Some(x),
                        _ => None,
                    };
                })
                .collect();
            return options.len() as Int;
        })
        .fold(1 as Int, |agg, next| agg * next);
}

fn part1(input: &str) -> Int {
    let lines: Vec<Vec<Int>> = input
        .trim()
        .split('\n')
        .map(|line| {
            return line
                .split(' ')
                .filter_map(|s| s.parse::<Int>().ok())
                .collect();
        })
        .collect();
    let lines_ref = &lines;
    return solve(&lines_ref[0], &lines_ref[1]);
}

fn part2(input: &str) -> Int {
    let lines: Vec<Vec<Int>> = input
        .trim()
        .split('\n')
        .map(|line| {
            line.split(':')
                .nth(1)
                .unwrap()
                .split(' ')
                .collect::<Vec<&str>>()
                .join("")
                .parse::<Int>()
                .unwrap()
        })
        .map(|single| vec![single])
        .collect();
    let lines_ref = &lines;
    return solve(&lines_ref[0], &lines_ref[1]);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::resource;
    static EXAMPLE: &str = r#"
Time:      7  15   30
Distance:  9  40  200
"#;

    #[test]
    fn part1_example() {
        let result = part1(&EXAMPLE);
        assert_eq!(288, result);
    }

    #[test]
    fn part1_result() {
        let input = resource("src/day06.txt");
        let result = part1(&input);
        assert_eq!(1660968, result);
    }

    #[test]
    fn part2_example() {
        let result = part2(&EXAMPLE);
        assert_eq!(71503, result);
    }

    #[test]
    fn part2_result() {
        let input = resource("src/day06.txt");
        let result = part2(&input);
        assert_eq!(26499773, result);
    }
}
