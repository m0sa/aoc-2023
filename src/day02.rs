use dict::{Dict, DictIface};
use regex::Regex;

struct Game {
    id: i32,
    hands: Vec<Dict<i32>>,
}

fn parse_hand(hand_str: &str) -> Dict<i32> {
    let hand_re = Regex::new(r"(?<count>\d+) (?<color>[^ ,;]+)").unwrap();
    let mut hand = Dict::<i32>::new();
    for color_cap in hand_re.captures_iter(hand_str) {
        let color_count: i32 = color_cap
            .name("count")
            .expect("no count")
            .as_str()
            .parse()
            .unwrap();

        let color_name = color_cap
            .name("color")
            .expect("no color")
            .as_str()
            .to_string();
        hand.add(color_name, color_count);
    }
    return hand;
}

fn parse_games(games_str: &str) -> Vec<Game> {
    let game_re = Regex::new(r"Game (?<id>\d+):(?<hands>.*)").unwrap();

    return games_str
        .split('\n')
        .map(|line| {
            let line_cap = game_re.captures(line).unwrap();
            let hands_cap = line_cap.name("hands").expect("no hands").as_str();

            let game_hands: Vec<Dict<i32>> = hands_cap.split(';').map(parse_hand).collect();

            let game_id: i32 = line_cap
                .name("id")
                .expect("invalid game id")
                .as_str()
                .parse()
                .unwrap();

            return Game {
                id: game_id,
                hands: game_hands,
            };
        })
        .collect();
}

fn part1(games_input: &str, constraint_input: &str) -> i32 {
    let constraint = parse_hand(constraint_input);

    return parse_games(games_input)
        .into_iter()
        .filter_map(|game| {
            if game.hands.into_iter().all(|hand| {
                hand.into_iter()
                    .all(|kvp| constraint.get(&kvp.key).unwrap() >= &kvp.val)
            }) {
                return Some(game.id);
            }

            return None;
        })
        .sum();
}

fn part2(games_input: &str) -> i32 {
    return parse_games(games_input)
        .into_iter()
        .map(|game| {
            let mut pow = Dict::<i32>::new();
            for hand in game.hands {
                for set in hand {
                    let color = set.key;
                    let value = set.val;
                    match pow.get(&color) {
                        Some(&existing) => {
                            pow.remove_key(&color);
                            pow.add(color, std::cmp::max(value, existing));
                        }
                        None => {
                            pow.add(color, value);
                        }
                    }
                }
            }
            return pow
                .into_iter()
                .fold(1, |agg, color_max| agg * color_max.val);
        })
        .sum();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    const PART1_CONSTRAINT: &str = "12 red, 13 green, 14 blue";
    const EXAMPLE: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
    #[test]
    fn part1_example() {
        let result = part1(EXAMPLE, PART1_CONSTRAINT);

        assert_eq!(8, result);
    }

    #[test]
    fn part1_result() {
        let input = utils::resource("src/day02.txt");
        let result = part1(&input, PART1_CONSTRAINT);
        assert_eq!(result, 2061);
    }

    #[test]
    fn part2_example() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 2286);
    }
    #[test]
    fn part2_result() {
        let input = utils::resource("src/day02.txt");
        let result = part2(&input);
        assert_eq!(result, 72596);
    }
}
