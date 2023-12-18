use crate::utils::*;
use std::collections::{BinaryHeap, HashMap, HashSet};
type Map = std::collections::HashMap<Point2D, u8>;

fn parse_map(input: &str) -> Map {
    let mut result = Map::new();
    for (y, line) in input.trim().split('\n').enumerate() {
        for (x, ch) in line.char_indices() {
            result.insert(
                Point2D::new(x as i32, y as i32),
                ch.to_digit(10).expect("needz moar digit") as u8,
            );
        }
    }
    return result;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Node {
    start: Point2D,
    direction: Point2D,
}

fn solve<T, I>(input: &str, next_directions: T, is_valid_solution: I) -> i32
where
    T: Fn(&Point2D) -> Vec<Point2D>,
    I: Fn(&Node) -> bool,
{
    let map = parse_map(input);
    let finish = Point2D::new(
        map.keys().map(|p| p.x).max().expect("max x"),
        map.keys().map(|p| p.y).max().expect("max y"),
    );
    let mut distances = HashMap::<Node, i32>::new();
    let mut visited = HashSet::<Node>::new();
    let mut queue = BinaryHeap::<(i32, Node)>::new();

    let right = Node {
        start: (0, 0).into(),
        direction: (1, 0).into(),
    };
    let down = Node {
        start: (0, 0).into(),
        direction: (0, 1).into(),
    };
    queue.push((0, right));
    queue.push((0, down));
    distances.insert(right, 0);
    distances.insert(down, 0);

    while let Some((_, current)) = queue.pop() {
        if !visited.insert(current) {
            continue;
        };

        for direction in next_directions(&current.direction) {
            let (_, bearing) = direction.manhattan_normalize();
            let next_pos = current.start + bearing;
            let move_cost = map.get(&next_pos);
            if move_cost.is_none() {
                continue; // out of bounds
            }
            let cost = *move_cost.unwrap() as i32;

            let next = Node {
                start: next_pos,
                direction,
            };
            let distance = distances
                .get(&next)
                .or(Some(&i32::MAX))
                .unwrap()
                .clone()
                .min(distances[&current] + cost);

            distances.insert(next, distance);
            queue.push((-distance, next)); // min queue
        }
    }

    distances
        .iter()
        .filter(|(k, _)| k.start == finish && is_valid_solution(k))
        .map(|(_, v)| *v)
        .min()
        .expect("no result")
}

fn part1(input: &str) -> i32 {
    solve(
        input,
        |direction| {
            let (steps, bearing) = direction.manhattan_normalize();
            let mut result = vec![bearing.clockwise(), bearing.counter_clockwise()];
            if steps < 3 {
                result.insert(0, direction + bearing);
            }
            return result;
        },
        |_| true,
    )
}

fn part2(input: &str) -> i32 {
    solve(
        input,
        |direction| {
            let (steps, bearing) = direction.manhattan_normalize();
            let mut result = vec![];
            if steps < 10 {
                result.push(bearing + direction);
            }
            if steps >= 4 {
                result.push(bearing.clockwise());
                result.push(bearing.counter_clockwise());
            }
            return result;
        },
        |node| node.direction.manhattan_len() >= 4,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    const EXAMPLE: &str = r"
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn part1_example() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 102)
    }

    #[test]
    fn part1_result() {
        let input = utils::resource("src/day17.txt");
        let result = part1(&input);
        assert_eq!(result, 758);
    }

    #[test]
    fn part2_example() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 94)
    }

    #[test]
    fn part2_result() {
        let input = utils::resource("src/day17.txt");
        let result = part2(&input);
        assert_eq!(result, 892);
    }
}
