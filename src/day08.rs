use regex::Regex;
use std::collections::{HashMap, HashSet};

type Graph<'a> = HashMap<&'a str, (&'a str, &'a str)>;

fn parse_input(input: &str) -> (Vec<char>, Graph) {
    let lines: Vec<&str> = input.trim().split('\n').collect();
    let parser = Regex::new(r"(?<node>.+) = \((?<left>[^,]+), (?<right>[^\)]+)\)").unwrap();
    let instructions: Vec<char> = lines[0].chars().collect();

    let mut graph = Graph::new();
    for ln in lines.into_iter().skip(2) {
        let cap = parser.captures(ln).unwrap();
        let name = cap.name("node").expect("node not parsed").as_str();
        let left = cap.name("left").expect("left not parsed").as_str();
        let right = cap.name("right").expect("right not parsed").as_str();
        graph.insert(name, (left, right));
    }
    let graph = graph;

    return (instructions, graph);
}

fn exit_path_length(
    start_node: &str,
    end_nodes: &HashSet<&str>,
    instructions: &Vec<char>,
    graph: &Graph,
) -> usize {
    let mut node = start_node;
    let mut step = 0;
    let instructions_len = instructions.len();
    while !end_nodes.contains(node) {
        let lr = *graph.get(node).expect(node);
        let step_idx = step % instructions_len;
        match instructions[step_idx] {
            'L' => (node, _) = lr,
            'R' => (_, node) = lr,
            x => panic!(
                "unknown instruction {} at step {} in node {}",
                x, step_idx, node
            ),
        }
        step += 1;
    }
    return step;
}

fn part1(input: &str) -> usize {
    let (instructions, graph) = parse_input(input);
    let exzzzit = &["ZZZ"].into_iter().collect::<HashSet<&str>>();
    return exit_path_length("AAA", exzzzit, &instructions, &graph);
}

fn part2(input: &str) -> u128 {
    let (instructions, graph) = parse_input(input);
    let end_nodes: HashSet<&str> = graph
        .keys()
        .filter(|k| k.ends_with("Z"))
        .map(|k| *k)
        .collect();
    return graph
        .keys()
        .filter(|k| k.ends_with("A"))
        .map(|k| exit_path_length(*k, &end_nodes, &instructions, &graph))
        .fold(1 as u128, |agg, next| num::integer::lcm(agg, next as u128));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;

    #[test]
    fn part1_example() {
        let result = part1(
            r#"
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#,
        );

        assert_eq!(6, result);
    }

    #[test]
    fn part1_result() {
        let result = part1(&resource("src/day08.txt"));

        assert_eq!(17873, result);
    }

    #[test]
    fn part2_example() {
        let result = part2(
            r#"
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
"#,
        );

        assert_eq!(6, result);
    }

    #[test]
    fn part2_result() {
        let result = part2(&resource("src/day08.txt"));

        assert_eq!(15746133679061, result);
    }
}
