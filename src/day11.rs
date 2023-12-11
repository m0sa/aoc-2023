type Int = u64;
type Point2D = (Int, Int);
use std::collections::HashSet;

fn solve(input: &str, expansion_rate: Int) -> Int {
    let mut galaxies = HashSet::<Point2D>::new();
    let lines: Vec<&str> = input.split('\n').collect();
    let void_cols: HashSet<usize> = (0..lines[0].len())
        .filter(|&x| (0..lines.len()).all(|y| lines[y].chars().nth(x) == Some('.')))
        .collect();
    let void_rows: HashSet<usize> = (0..lines.len())
        .filter(|&y| lines[y].find('#').is_none())
        .collect();
    let mut expand_y: Int = 0;
    for (y, line) in lines.iter().enumerate() {
        if void_rows.contains(&y) {
            expand_y = expand_y + expansion_rate - 1;
            continue;
        }

        let mut expand_x = 0;
        for (x, ch) in line.char_indices() {
            if void_cols.contains(&x) {
                expand_x = expand_x + expansion_rate - 1;
            }
            if ch == '#' {
                galaxies.insert((x as Int + expand_x, y as Int + expand_y));
            }
        }
    }
    let galaxies = galaxies;

    let mut pairs = Vec::<(Point2D, Point2D)>::new();
    let pairq: Vec<Point2D> = galaxies.into_iter().collect();
    for cur in 0..pairq.len() {
        for nxt in cur + 1..pairq.len() {
            pairs.push((pairq[cur], pairq[nxt]));
        }
    }
    let pairs = pairs;

    pairs
        .iter()
        .map(|(a, b)| {
            (a.0 as i128 - b.0 as i128).abs() as Int + (a.1 as i128 - b.1 as i128).abs() as Int
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;

    static EXAMPLE: &str = r#"
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
"#;
    #[test]
    fn part1_example() {
        let result = solve(EXAMPLE.trim(), 2);
        assert_eq!(374, result);
    }

    #[test]
    fn part1_result() {
        let result = solve(&resource("src/day11.txt"), 2);
        assert_eq!(9591768, result);
    }

    #[test]
    fn part2_example() {
        assert_eq!(1030, solve(EXAMPLE.trim(), 10));
        assert_eq!(8410, solve(EXAMPLE.trim(), 100));
    }

    #[test]
    fn part2_result() {
        let result = solve(&resource("src/day11.txt"), 1_000_000);
        assert_eq!(746962097860, result);
    }
}
