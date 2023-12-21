use crate::utils::{Direction, Point2D};
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

fn part1_loop(
    map: &Map,
    frontier_current: &mut Coordinates,
    frontier_next: &mut Coordinates,
    visited: &mut Coordinates,
) {
    for current in frontier_current.drain() {
        for direction in Direction::ALL {
            let next = current.translate(*direction);
            if let Some(Tile::Garden) = map.get(&next) {
                if visited.insert(next) {
                    frontier_next.insert(next);
                }
            }
        }
    }
}

fn part1(input: &str, iterations: usize) -> Int {
    let (start, map) = parse_input(input);

    let ref mut visited_even = Coordinates::new();
    let ref mut visited_odd = Coordinates::new();
    let ref mut frontier_even = Coordinates::new();
    let ref mut frontier_odd = Coordinates::new();

    frontier_even.insert(start);

    for i in 0..iterations {
        if i % 2 == 0 {
            part1_loop(&map, frontier_even, frontier_odd, visited_even);
        } else {
            part1_loop(&map, frontier_odd, frontier_even, visited_odd);
        };
    }

    if iterations % 2 == 0 {
        visited_odd.len() as Int
    } else {
        visited_even.len() as Int
    }
}

fn part2(input: &str) -> Int {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn part1_example1() {
        let result = part1(
            r#"
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
"#,
            6,
        );
        assert_eq!(result, 16)
    }

    #[test]
    fn part1_result() {
        let input = utils::resource("src/day21.txt");
        let result = part1(&input, 64);
        assert_eq!(result, 3830);
    }

    #[test]
    fn part2_result() {
        let input = utils::resource("src/day21.txt");
        let result = part2(&input);
        assert_eq!(result, 0);
    }
}
