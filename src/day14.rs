type Int = usize;
use crate::utils::Point2D;
use std::collections::HashMap;

type Map = HashMap<Point2D, Stone>;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
enum Stone {
    Fixed,
    Rolling,
}

fn parse_input(input: &str) -> (Point2D, Map) {
    let mut result = Map::new();
    let mut bounds = Point2D::new(0, 0);
    for (y, line) in input.split('\n').enumerate() {
        for (x, ch) in line.char_indices() {
            let point = Point2D::new(x as i32, y as i32);
            match ch {
                '.' => {}
                '#' => {
                    result.insert(point, Stone::Fixed);
                }
                'O' => {
                    result.insert(point, Stone::Rolling);
                }
                c => panic!("unexpected character {}", c),
            };
            bounds = point;
        }
    }
    return (bounds + Point2D::new(1, 1), result);
}

fn tilt_north(bounds: Point2D, map: &Map) -> Map {
    let mut result = map.clone();
    for y in 0..bounds.y {
        for x in 0..bounds.x {
            let p = Point2D::new(x, y);
            if result.get(&p).is_none() {
                // since we start at y 0, we don't need to look above, we just need to look bellow, if there's something we can roll into this position
                for z in y + 1..bounds.y {
                    let p_next = Point2D::new(x, z);
                    let p_stone = result.get(&p_next);
                    match p_stone {
                        None => {
                            continue; // next
                        }
                        Some(Stone::Fixed) => {
                            break; // no need to look past it, we're done
                        }
                        Some(Stone::Rolling) => {
                            result.remove(&p_next);
                            result.insert(p, Stone::Rolling);
                            // roll the stone north!
                            break;
                        }
                    }
                }
            }
        }
    }
    return result;
}

pub fn part1(input: &str) -> Int {
    let (bounds, map) = parse_input(input);
    let rolled = tilt_north(bounds, &map);
    return rolled
        .iter()
        .filter_map(|(position, &stone)| match stone {
            Stone::Fixed => None,
            Stone::Rolling => Some(bounds.y - position.y),
        })
        .sum::<i32>() as Int;
}

pub fn part2(input: &str) -> Int {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn part1_example() {
        let example = r#"
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
"#
        .trim();
        let result = part1(example);
        assert_eq!(result, 136)
    }

    #[test]
    fn part1_result() {
        let input = utils::resource("src/day14.txt");
        let result = part1(&input);
        assert_eq!(result, 105982);
    }

    #[test]
    fn part2_example() {
        let example = r#"

"#
        .trim();
        let result = part2(example);
        assert_eq!(result, 0)
    }

    #[test]
    fn part2_result() {
        let input = utils::resource("src/day14.txt");
        let result = part2(&input);
        assert_eq!(result, 0);
    }
}
