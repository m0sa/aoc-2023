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

fn tilt_north(bounds: &Point2D, map: &mut Map) {
    for y in 0..bounds.y {
        for x in 0..bounds.x {
            let p = Point2D::new(x, y);
            if map.get(&p).is_none() {
                // since we start at y 0, we don't need to look above, we just need to look bellow, if there's something we can roll into this position
                for z in y + 1..bounds.y {
                    let p_next = Point2D::new(x, z);
                    match map.get(&p_next) {
                        None => {
                            continue; // next
                        }
                        Some(Stone::Fixed) => {
                            break; // no need to look past it, we're done
                        }
                        Some(Stone::Rolling) => {
                            map.remove(&p_next);
                            map.insert(p, Stone::Rolling);
                            // roll the stone north!
                            break;
                        }
                    }
                }
            }
        }
    }
}

fn score(bounds: &Point2D, map: &Map) -> Int {
    return map
        .iter()
        .filter_map(|(position, &stone)| match stone {
            Stone::Fixed => None,
            Stone::Rolling => Some(bounds.y - position.y),
        })
        .sum::<i32>() as Int;
}

fn part1(input: &str) -> Int {
    let (bounds, mut map) = parse_input(input);
    tilt_north(&bounds, &mut map);
    return score(&bounds, &map);
}

fn rotate_clockwise(bounds: &mut Point2D, map: &mut Map) {
    let stones = map
        .iter()
        .map(|(&pos, &stone)| (pos, stone))
        .collect::<Vec<(Point2D, Stone)>>();
    let old_bounds = bounds.clone();
    bounds.x = old_bounds.y;
    bounds.y = old_bounds.x;

    map.clear();
    for (p, s) in stones {
        let y_ = p.x;
        let x_ = old_bounds.y - p.y - 1;
        let new_position = Point2D::new(x_, y_);
        map.insert(new_position, s);
    }
}

fn hash_key<'a>(bounds: &'a Point2D, map: &'a Map, s: &'a mut String) {
    for y in 0..bounds.y {
        for x in 0..bounds.x {
            s.push(match map.get(&Point2D::new(x, y)) {
                None => '.',
                Some(Stone::Fixed) => '#',
                Some(Stone::Rolling) => 'O',
            });
        }
        s.push('\n');
    }
}
const CYCLES: usize = 1000000000;
fn part2(input: &str) -> Int {
    let (mut bounds, mut map) = parse_input(input);
    let mut c = 0;
    let mut cache = HashMap::<String, usize>::new();

    while c < CYCLES {
        let mut key = String::new();
        hash_key(&bounds, &map, &mut key);

        let cached = *cache.entry(key).or_insert(c.clone());
        if cached < c {
            let increment = c - cached;
            let remaining = CYCLES - c;
            let increment = increment * (remaining / increment);
            c += increment;

            // panic!("hit loop at {c}, seen before at {cached}. Increasing c to {next}");
            if increment > 0 {
                continue;
            }
        }

        // a cycle is tilting north, west, south, east
        // instead of implementing each direction we can rotate and tilt north for each direction, untill we make a full circle
        for _ in &["N", "W", "S", "E"] {
            tilt_north(&bounds, &mut map);
            rotate_clockwise(&mut bounds, &mut map);
        }
        c += 1;
    }
    score(&bounds, &map)
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
        let result = part2(example);
        assert_eq!(result, 64)
    }

    #[test]
    fn part2_result() {
        let input = utils::resource("src/day14.txt");
        let result = part2(&input);
        assert_eq!(result, 85175);
    }
}
