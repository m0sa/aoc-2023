use crate::utils::*;
use std::collections::{HashMap, HashSet};
type Int = usize;
type Map = HashMap<Point2D, Tile>;
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Tile {
    Garden,
    Rocks,
}

fn parse_input(input: &str) -> (Point2D, Map) {
    let mut map = Map::new();
    let mut start = Point2D::zero();
    for (j, line) in input.trim().split('\n').enumerate() {
        for (i, ch) in line.char_indices() {
            let point = Point2D::new(i as i32, j as i32);
            match ch {
                '.' => {
                    map.insert(point, Tile::Garden);
                }
                '#' => {
                    map.insert(point, Tile::Rocks);
                }
                'S' => {
                    map.insert(point, Tile::Garden);
                    start = point;
                }
                u => panic!("unexpected character in input '{}'", u),
            }
        }
    }
    (start, map)
}

type Coordinates = HashSet<Point2D>;

fn main_loop<T>(
    get_tile: T,
    frontier_current: &mut Coordinates,
    frontier_next: &mut Coordinates,
    visited: &mut Coordinates,
) where
    T: FnOnce(Point2D) -> Option<Tile> + std::marker::Copy,
{
    for current in frontier_current.drain() {
        for direction in Direction::ALL {
            let next = current.translate(*direction);
            if let Some(Tile::Garden) = get_tile(next) {
                if visited.insert(next) {
                    frontier_next.insert(next);
                }
            }
        }
    }
}

fn solve<T, E, M>(steps: usize, start: &Point2D, get_tile: T, post_step: E, memory: &mut M) -> Int
where
    T: FnOnce(Point2D) -> Option<Tile> + std::marker::Copy,
    E: FnOnce(usize, &Coordinates, &Coordinates, &Coordinates, &mut M) -> Option<Int>
        + std::marker::Copy,
{
    let ref mut visited_even = Coordinates::new();
    let ref mut visited_odd = Coordinates::new();
    let ref mut frontier_even = Coordinates::new();
    let ref mut frontier_odd = Coordinates::new();

    frontier_even.insert(*start);

    for i in 0..steps {
        let result = if i % 2 == 0 {
            main_loop(get_tile, frontier_even, frontier_odd, visited_even);
            post_step(i + 1, visited_even, visited_odd, frontier_odd, memory)
        } else {
            main_loop(get_tile, frontier_odd, frontier_even, visited_odd);
            post_step(i + 1, visited_odd, visited_even, frontier_even, memory)
        };
        if let Some(value) = result {
            return value;
        }
    }

    panic!("no result!")
}

fn part1(input: &str, steps: usize) -> Int {
    let (start, map) = parse_input(input);

    solve(
        steps,
        &start,
        |p| map.get(&p).map(|x| *x),
        |s, visited, _, _, _| {
            if s == steps {
                Some(visited.len() as Int)
            } else {
                None
            }
        },
        &mut "".to_string(),
    )
}

fn from_repeated(p: &Point2D, mod_x: i32, mod_y: i32) -> Point2D {
    Point2D::new(positive_modulo(p.x, mod_x), positive_modulo(p.y, mod_y))
}

fn part2(input: &str, steps: usize) -> Int {
    let (start, map) = parse_input(input);
    let max_x = map.keys().map(|p| p.x as usize).max().unwrap() + 1;
    let max_y = map.keys().map(|p| p.y).max().unwrap() + 1;

    let ref mut memory = (
        (0..max_x).map(|_| 0).collect::<Vec<i64>>(),
        (0..max_x).map(|_| 0).collect::<Vec<i64>>(),
        (0..max_x).map(|_| 0).collect::<Vec<i64>>(),
    );
    solve(
        steps,
        &start,
        |p| map.get(&from_repeated(&p, max_x as i32, max_y)).map(|x| *x),
        |step, visited, visited_prev, frontier, mem| {
            let ref mut frontiers = mem.0;
            let ref mut frontier_increments = mem.1;
            let ref mut frontier_jitter = mem.2;

            if step == steps {
                return Some(visited.len());
            }

            let fsize = frontier.len() as i64;
            let ix = (step - 1) % max_x;
            if step > max_x {
                frontier_jitter[ix] = fsize - frontiers[ix] - frontier_increments[ix];
                frontier_increments[ix] = fsize - frontiers[ix];
            }
            frontiers[ix] = fsize;

            if step >= 2 * max_x && frontier_jitter.iter().all(|&i| i == 0) {
                let mut result_prev = visited_prev.len();
                let mut result = visited.len();
                for s in step..steps {
                    let ix = s % max_x;
                    frontiers[ix] += frontier_increments[ix];
                    let result_next = result_prev + frontiers[ix] as usize;
                    result_prev = result;
                    result = result_next;
                }
                return Some(result);
            }
            return None;
        },
        memory,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    const EXAMPLE: &str = r#"
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
"#;

    #[test]
    fn part1_example1() {
        let result = part1(EXAMPLE, 6);
        assert_eq!(result, 16)
    }

    #[test]
    fn part1_result() {
        let input = utils::resource("src/day21.txt");
        let result = part1(&input, 64);
        assert_eq!(result, 3830);
    }

    #[test]
    fn part2_example_6() {
        assert_eq!(part2(&EXAMPLE, 6), 16);
    }

    #[test]
    fn part2_example_10() {
        assert_eq!(part2(&EXAMPLE, 10), 50);
    }

    #[test]
    fn part2_example_50() {
        assert_eq!(part2(&EXAMPLE, 50), 1594);
    }

    #[test]
    fn part2_example_100() {
        assert_eq!(part2(&EXAMPLE, 100), 6536);
    }

    #[test]
    fn part2_example_500() {
        assert_eq!(part2(&EXAMPLE, 500), 167004);
    }

    #[test]
    fn part2_example_1000() {
        assert_eq!(part2(&EXAMPLE, 1000), 668697);
    }

    #[test]
    fn part2_example_5000() {
        assert_eq!(part2(&EXAMPLE, 5000), 16733044);
    }

    #[test]
    fn part2_result() {
        let input = utils::resource("src/day21.txt");
        assert_eq!(part2(&input, 26501365), 637087163925555);
    }
}
