use std::collections::HashMap;

use crate::utils::*;
type Int = usize;

struct Map {
    coordinates: HashMap<Point2D, char>,
    start: Point2D,
    bound: Point2D,
}
impl Map {
    fn new(text: &str) -> Map {
        let mut coordinates = HashMap::<Point2D, char>::new();
        let mut start = Point2D::new(0, 0);

        let lines: Vec<&str> = text.split('\n').collect();
        for y in 0..lines.len() {
            for x in 0..lines[y].len() {
                let ch = lines[y].chars().nth(x).unwrap();
                let pt = Point2D::new(x as i32, y as i32);
                match ch {
                    'S' => start = pt,
                    '.' => continue,
                    _ => {}
                }
                coordinates.insert(pt, ch);
            }
        }

        return Map {
            coordinates,
            start,
            bound: Point2D::new(lines[0].len() as i32, lines.len() as i32),
        };
    }

    fn at(&self, position: &Point2D) -> char {
        return match self.coordinates.get(position) {
            Some(&c) => c,
            None => '.',
        };
    }

    fn next(&self, from: &Point2D) -> Vec<Point2D> {
        let mut result = Vec::<Point2D>::new();
        let current = self.at(from);
        match current {
            // - is a horizontal pipe connecting east and west.
            '-' => {
                result.push(Self::east(from));
                result.push(Self::west(from));
            }
            // L is a 90-degree bend connecting north and east.
            'L' => {
                result.push(Self::north(from));
                result.push(Self::east(from));
            }
            // J is a 90-degree bend connecting north and west
            'J' => {
                result.push(Self::north(from));
                result.push(Self::west(from));
            }
            // 7 is a 90-degree bend connecting south and west.
            '7' => {
                result.push(Self::south(from));
                result.push(Self::west(from));
            }
            // F is a 90-degree bend connecting south and east.
            'F' => {
                result.push(Self::south(from));
                result.push(Self::east(from));
            }
            // | is a vertical pipe connecting north and south.
            '|' => {
                result.push(Self::north(from));
                result.push(Self::south(from));
            }
            'S' => {}
            '.' => {}
            x => panic!("unexpected: '{}'", x as char),
            // S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
            // . is ground; there is no pipe in this tile.
        }
        return result;
    }
    fn west(p: &Point2D) -> Point2D {
        p + Point2D::new(-1, 0)
    }
    fn east(p: &Point2D) -> Point2D {
        p + Point2D::new(1, 0)
    }
    fn north(p: &Point2D) -> Point2D {
        p + Point2D::new(0, -1)
    }
    fn south(p: &Point2D) -> Point2D {
        p + Point2D::new(0, 1)
    }
}

fn solve(input: &str) -> (Int, Int) {
    let map = &Map::new(input);
    let mut loop_coordinates = HashMap::<Point2D, char>::new();
    let mut explore = vec![
        Map::north(&map.start),
        Map::south(&map.start),
        Map::east(&map.start),
        Map::west(&map.start),
    ]
    .into_iter()
    .filter(|p| map.next(p).iter().any(|&s| s == map.start))
    .collect::<Vec<Point2D>>();

    while explore.len() > 0 {
        let cur = explore.pop().unwrap();
        for next in map.next(&cur) {
            if !loop_coordinates.contains_key(&next) {
                loop_coordinates.insert(next, map.at(&next));
                explore.push(next);
            }
        }
    }
    let part1 = loop_coordinates.len() / 2;

    let map = Map {
        bound: map.bound,
        coordinates: loop_coordinates,
        start: map.start,
    };
    let mut cnt = 0;
    for y in 0..map.bound.y {
        let mut pipe_crossings = 0;
        for x in 0..map.bound.x {
            let pos = Point2D::new(x, y);
            let ch = map.at(&pos);
            if ch == '.' && pipe_crossings % 2 == 1 {
                cnt = cnt + 1;
            } else if ch == '|' || ch == 'L' || ch == 'J' {
                pipe_crossings = pipe_crossings + 1;
            }
        }
    }
    let part2 = cnt;
    return (part1, part2);
}

fn part1(input: &str) -> Int {
    solve(input).0
}

fn part2(input: &str) -> Int {
    solve(input).1
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = r#"
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
    "#;
    #[test]
    fn basics() {
        let g = Map::new(EXAMPLE);
        assert_eq!(g.at(&g.start), 'S');
    }

    #[test]
    fn part1_example() {
        let result = part1(EXAMPLE.trim());
        assert_eq!(8, result);
    }

    #[test]
    fn part1_result() {
        let result = part1(&resource("src/day10.txt"));
        assert_eq!(7005, result);
    }

    #[test]
    fn part2_example1() {
        let result = part2(
            r#"
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
        "#
            .trim(),
        );
        assert_eq!(8, result);
    }

    #[test]
    fn part2_example2() {
        let result = part2(
            r#"
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
        "#
            .trim(),
        );
        assert_eq!(10, result);
    }

    #[test]
    fn part2_result() {
        let result = part2(&resource("src/day10.txt"));
        assert_eq!(417, result);
    }
}
