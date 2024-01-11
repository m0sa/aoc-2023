use crate::utils::*;
use core::panic;
use std::collections::HashMap;
type Int = u128;
type Part = [Int; 4];
const PART_LAYOUT: &str = "xmas";

enum Rule {
    Goto(String),
    If(Condition, String),
}
struct Condition {
    part_category: usize,
    operator: char,
    value: Int,
}
type Workflow = Vec<Rule>;
type System = HashMap<String, Workflow>;

fn evaluate(condition: &Condition, part: &Part) -> bool {
    match condition.operator {
        '>' => part[condition.part_category] > condition.value,
        '<' => part[condition.part_category] < condition.value,
        x => panic!("unimplemented operator '{}'", x),
    }
}

fn accept_or_reject(system: &System, part: &Part) -> bool {
    let mut workflow_name = "in".to_string();
    loop {
        if workflow_name == "A" {
            return true;
        } else if workflow_name == "R" {
            return false;
        } else if let Some(workflow) = system.get(&workflow_name) {
            for rule in workflow {
                match rule {
                    Rule::Goto(next) => {
                        workflow_name = next.clone();
                        break;
                    }
                    Rule::If(condition, next) => {
                        if evaluate(condition, part) {
                            workflow_name = next.clone();
                            break;
                        }
                    }
                }
            }
        } else {
            panic!("unexcpected workflow {}", workflow_name);
        }
    }
}

fn parse_system(input: &str) -> System {
    let mut system = System::new();
    for line in input.split('\n') {
        let rules_start = line.find('{').unwrap();
        let line_rules = line[rules_start + 1..]
            .trim_end_matches('}')
            .split(',')
            .map(|rule_str| {
                if let Some(then_pos) = rule_str.find(':') {
                    let then_str = rule_str[then_pos + 1..].to_string();
                    let part_char = &rule_str[0..1];
                    let part_category = PART_LAYOUT.find(part_char).unwrap();
                    let operator = rule_str[1..2].chars().nth(0).unwrap();
                    let value = rule_str[2..then_pos].parse::<Int>().unwrap();

                    Rule::If(
                        Condition {
                            part_category,
                            operator,
                            value,
                        },
                        then_str,
                    )
                } else {
                    Rule::Goto(rule_str.to_string())
                }
            })
            .collect::<Vec<Rule>>();
        let name = line[0..rules_start].to_string();
        system.insert(name, line_rules);
    }
    return system;
}

fn part1(input: &str) -> Int {
    let input_parts = input.trim().split("\n\n").collect::<Vec<&str>>();

    let system = parse_system(input_parts[0]);

    let mut part_ratings = Vec::<Part>::new();
    for line in input_parts[1].split('\n') {
        let ints = line
            .split(',')
            .map(|p| {
                p[p.find('=').unwrap() + 1..]
                    .trim_end_matches('}')
                    .parse::<Int>()
                    .unwrap()
            })
            .collect::<Vec<Int>>();
        part_ratings.push(ints.try_into().unwrap());
    }
    let part_ratings = part_ratings;
    part_ratings
        .into_iter()
        .filter(|p| accept_or_reject(&system, p))
        .map(|p| p.into_iter().sum::<Int>())
        .sum()
}

type Wave = [Point2D; 4];

fn part2(input: &str) -> Int {
    let input_parts = input.trim().split("\n\n").collect::<Vec<&str>>();
    let system = parse_system(input_parts[0]);

    let mut work = Vec::<(String, Wave)>::new();
    work.push((
        "in".to_string(),
        [
            (1, 4000).into(),
            (1, 4000).into(),
            (1, 4000).into(),
            (1, 4000).into(),
        ],
    ));
    let mut accepted = vec![];

    while let Some((workflow_name, mut wave_reamining)) = work.pop() {
        if workflow_name == "A" {
            accepted.push(wave_reamining);
        } else if workflow_name == "R" {
        } else if let Some(workflow) = system.get(&workflow_name) {
            for rule in workflow {
                match rule {
                    Rule::Goto(next) => {
                        work.push((next.clone(), wave_reamining));
                    }
                    Rule::If(condition, next) => {
                        let range = wave_reamining[condition.part_category];
                        let (start, end) = (range.x, range.y);
                        let val = condition.value as i32;

                        let mut matching: Option<Point2D> = None;
                        let mut remaining: Option<Point2D> = None;
                        match condition.operator {
                            '>' => {
                                if start > val {
                                    matching = Some(range);
                                } else if end < val {
                                    remaining = Some(range);
                                } else {
                                    matching = Some(Point2D::new(val + 1, end));
                                    remaining = Some(Point2D::new(start, val));
                                }
                            }
                            '<' => {
                                if end < val {
                                    matching = Some(range);
                                } else if start > val {
                                    remaining = Some(range);
                                } else {
                                    matching = Some(Point2D::new(start, val - 1));
                                    remaining = Some(Point2D::new(val, end));
                                }
                            }
                            x => panic!("unexpected operator {}", x),
                        };

                        if matching.is_some() {
                            wave_reamining[condition.part_category] = matching.unwrap();
                            work.push((next.clone(), wave_reamining));
                        }

                        if remaining.is_some() {
                            wave_reamining[condition.part_category] = remaining.unwrap();
                        }
                    }
                }
            }
        } else {
            panic!("unexcpected workflow {}", workflow_name);
        }
    }

    return accepted
        .iter()
        .map(|wave| wave.iter().fold(1, |agg, r| agg * (r.y - r.x + 1) as Int))
        .sum::<Int>();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    const EXAMPLE: &str = r"
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
";

    #[test]
    fn part1_example() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 19114)
    }

    #[test]
    fn part1_result() {
        let input = utils::resource("src/day19.txt");
        let result = part1(&input);
        assert_eq!(result, 373302);
    }

    #[test]
    fn part2_example() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 167409079868000)
    }

    #[test]
    fn part2_result() {
        let input = utils::resource("src/day19.txt");
        let result = part2(&input);
        assert_eq!(result, 130262715574114);
    }
}
