type Int = isize;

fn solve<T>(input: &str, func: T) -> Int
where
    T: Fn(Vec<Int>) -> Int,
{
    let result = input
        .split('\n')
        .map(|line| {
            line.split(' ')
                .map(|x| x.parse::<Int>().unwrap())
                .collect::<Vec<Int>>()
        })
        .map(|x| func(x))
        .sum();
    return result;
}

fn diff_line(line: Vec<Int>) -> Vec<Int> {
    let mut diff = Vec::<Int>::new();
    for i in 1..line.len() {
        let prev = line[i - 1];
        let cur = line[i];
        diff.push(cur - prev);
    }
    return diff;
}

fn solve_line_part1(line: Vec<Int>) -> Int {
    if line.iter().all(|&x| x == 0) {
        return 0;
    }
    let above_val = line.last().unwrap().clone();
    let below_val = solve_line_part1(diff_line(line));
    return above_val + below_val;
}

fn part1(input: &str) -> Int {
    solve(input, solve_line_part1)
}

fn solve_line_part2(line: Vec<Int>) -> Int {
    if line.iter().all(|&x| x == 0) {
        return 0;
    }
    let above_val = line.first().unwrap().clone();
    let below_val = solve_line_part2(diff_line(line));
    return above_val - below_val;
}

fn part2(input: &str) -> Int {
    solve(input, solve_line_part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;

    static EXAMPLE: &str = r#"
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
    "#;
    #[test]
    fn part1_example() {
        let result = part1(EXAMPLE.trim());
        assert_eq!(114, result);
    }

    #[test]
    fn part1_result() {
        let result = part1(&resource("src/day09.txt"));
        assert_eq!(1904165718, result);
    }

    #[test]
    fn part2_example() {
        let result = part2(EXAMPLE.trim());
        assert_eq!(2, result);
    }

    #[test]
    fn part2_result() {
        let result = part2(&resource("src/day09.txt"));
        assert_eq!(964, result);
    }
}
