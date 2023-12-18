use crate::utils::Point2D;
use std::collections::{HashMap, HashSet};
type Map = HashMap<Point2D, char>;
type Beam = (Point2D, Direction);

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Left,
    Down,
}
fn translate(current: &Point2D, direction: Direction) -> Point2D {
    current
        + match direction {
            Direction::Up => Point2D::new(0, -1),
            Direction::Right => Point2D::new(1, 0),
            Direction::Down => Point2D::new(0, 1),
            Direction::Left => Point2D::new(-1, 0),
        }
}
fn direct_beam(tile_type: char, incoming_direction: Direction) -> Vec<Direction> {
    match tile_type {
        '.' => vec![incoming_direction],
        // mirrors
        '/' => match incoming_direction {
            Direction::Up => vec![Direction::Right],
            Direction::Right => vec![Direction::Up],
            Direction::Down => vec![Direction::Left],
            Direction::Left => vec![Direction::Down],
        },
        '\\' => match incoming_direction {
            Direction::Up => vec![Direction::Left],
            Direction::Right => vec![Direction::Down],
            Direction::Down => vec![Direction::Right],
            Direction::Left => vec![Direction::Up],
        },
        // splitters
        '-' => match incoming_direction {
            Direction::Up => vec![Direction::Left, Direction::Right],
            Direction::Right => vec![Direction::Right],
            Direction::Down => vec![Direction::Left, Direction::Right],
            Direction::Left => vec![Direction::Left],
        },
        '|' => match incoming_direction {
            Direction::Up => vec![Direction::Up],
            Direction::Right => vec![Direction::Up, Direction::Down],
            Direction::Down => vec![Direction::Down],
            Direction::Left => vec![Direction::Up, Direction::Down],
        },
        c => panic!("expected tile, got '{}'", c),
    }
}

fn parse_input(input: &str) -> Map {
    let mut map = Map::new();
    for (y, line) in input.trim().split('\n').enumerate() {
        for (x, ch) in line.char_indices() {
            map.insert((x as i32, y as i32).into(), ch);
        }
    }
    return map;
}

fn count_energized(map: &Map, start_at: Beam) -> usize {
    let mut cur_beams = vec![start_at];
    let mut visited = HashSet::<Beam>::new();

    while cur_beams.len() > 0 {
        let mut next_beams = Vec::<Beam>::new();

        for beam in cur_beams {
            if visited.contains(&beam) {
                continue; // already calculated
            }

            let (b_pos, b_dir) = beam;
            let tile = map.get(&b_pos);
            if tile.is_none() {
                // out of bounds
                continue;
            }

            visited.insert(beam);
            for next_direction in direct_beam(*tile.unwrap(), b_dir) {
                let next_position = translate(&b_pos, next_direction);
                next_beams.push((next_position, next_direction));
            }
        }

        cur_beams = next_beams;
    }

    let visited_positions: HashSet<Point2D> = visited.iter().map(|(p, _)| *p).collect();
    return visited_positions.len();
}

fn part1(input: &str) -> usize {
    count_energized(&parse_input(input), (Point2D::new(0, 0), Direction::Right))
}

fn part2(input: &str) -> usize {
    let map = parse_input(input);

    let mut potential_starts = Vec::<Beam>::new();
    let max_x = map.keys().map(|p| p.x).max().unwrap();
    let max_y = map.keys().map(|p| p.y).max().unwrap();
    for x in 0..=max_x {
        potential_starts.push((Point2D::new(x, 0), Direction::Down));
        potential_starts.push((Point2D::new(x, max_y), Direction::Up));
    }
    for y in 0..=max_y {
        potential_starts.push((Point2D::new(0, y), Direction::Right));
        potential_starts.push((Point2D::new(max_x, y), Direction::Left));
    }

    potential_starts
        .into_iter()
        .map(|b| count_energized(&map, b))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    const EXAMPLE: &str = r"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
    #[test]
    fn part1_example() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 46)
    }

    #[test]
    fn part1_result() {
        let input = utils::resource("src/day16.txt");
        let result = part1(&input);
        assert_eq!(result, 7415);
    }

    #[test]
    fn part2_example() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 51)
    }

    #[test]
    fn part2_result() {
        let input = utils::resource("src/day16.txt");
        let result = part2(&input);
        assert_eq!(result, 7943);
    }
}
