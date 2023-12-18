use crate::utils::*;
type Int = i64;

fn area(edges: Vec<Point2D>) -> Int {
    let row_area: Int = edges
        .iter()
        .zip(edges.iter().cycle().skip(1))
        .map(|(a, b)| ((a * -1) + b).manhattan_len() as Int)
        .sum::<Int>();
    let shoelace_area: Int = edges
        .iter()
        .zip(edges.iter().cycle().skip(1))
        .map(|(a, b)| a.x as Int * b.y as Int - a.y as Int * b.x as Int)
        .sum::<Int>()
        .abs()
        / 2;
    (row_area / 2 + 1) + shoelace_area
}

fn part1(input: &str) -> Int {
    let mut edges = vec![];
    let mut current = Point2D::zero();
    edges.push(current);
    for line in input.trim().split('\n') {
        let items: Vec<&str> = line.split(' ').collect();
        let direction = match items[0] {
            "U" => Direction::Up,
            "R" => Direction::Right,
            "D" => Direction::Down,
            "L" => Direction::Left,
            x => panic!("unexpected direction: {}", x),
        };
        let steps = items[1].parse::<i32>().expect("NaN");
        let next = Point2D::zero().translate(direction) * steps as i32;
        current = current + next;
        edges.push(current);
    }

    area(edges)
}

fn part2(input: &str) -> Int {
    let mut edges = vec![];
    let mut current = Point2D::zero();
    edges.push(current);
    for line in input.trim().split('\n') {
        let items: Vec<&str> = line.split(' ').collect();
        let steps_hex = &items[2][2..7];
        let steps = i32::from_str_radix(steps_hex, 16).expect("invalid number");

        let direction = match &items[2][7..8] {
            "0" => Direction::Right,
            "1" => Direction::Down,
            "2" => Direction::Left,
            "3" => Direction::Up,
            x => panic!("unexpected direction: {}", x),
        };
        let next = Point2D::zero().translate(direction) * steps;
        current = current + next;
        edges.push(current);
    }

    area(edges)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    const EXAMPLE: &str = r"
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn part1_example() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 62)
    }

    #[test]
    fn part1_result() {
        let input = utils::resource("src/day18.txt");
        let result = part1(&input);
        assert_eq!(result, 48652);
    }

    #[test]
    fn part2_example() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 952408144115)
    }

    #[test]
    fn part2_result() {
        let input = utils::resource("src/day18.txt");
        let result = part2(&input);
        assert_eq!(result, 45757884535661);
    }
}
