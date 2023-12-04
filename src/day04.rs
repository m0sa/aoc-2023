use std::collections::HashSet;

fn parse_numbers<'a>(numbers_str: &'a str) -> HashSet<i32> {
    return numbers_str
        .split(' ')
        .filter_map(|s| match s.parse::<i32>() {
            Ok(x) => Some(x),
            _ => None,
        })
        .collect();
}

fn winners(input: &str) -> Vec<usize> {
    input
        .split('\n')
        .map(|ln| {
            let id_and_sets: Vec<&str> = ln.split(':').collect();
            let number_sets: Vec<&str> = id_and_sets[1].split('|').collect();

            let win_set = parse_numbers(number_sets[0]);
            let my_set = parse_numbers(number_sets[1]);
            return my_set.into_iter().filter(|my| win_set.contains(my)).count();
        })
        .collect()
}

fn part1(input: &str) -> i32 {
    let winners = winners(input);
    let base_two: i32 = 2;
    return winners
        .iter()
        .map(|&w| {
            if w == 0 {
                0
            } else {
                base_two.pow(w as u32 - 1)
            }
        })
        .sum();
}

fn part2(input: &str) -> u32 {
    let winners = winners(input);

    let mut counts: Vec<u32> = winners.iter().map(|_| 1).collect();
    let card_counts_len = counts.len();

    let mut card_id = 0 as usize;
    loop {
        if card_id >= card_counts_len {
            break;
        }
        let next = card_id + 1;
        let cards_won: usize = winners[card_id];

        for win_id in next..(next + cards_won) {
            if win_id >= card_counts_len {
                break;
            }
            counts[win_id] += counts[card_id];
        }

        card_id = next;
    }

    counts.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    const EXAMPLE: &str = r#"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"#;

    #[test]
    fn part1_example() {
        let result = part1(EXAMPLE.trim());
        assert_eq!(13, result);
    }
    #[test]
    fn part1_result() {
        let input = utils::resource("src/day04.txt");
        let result = part1(&input);

        assert_eq!(25183, result);
    }
    #[test]
    fn part2_example() {
        let result = part2(EXAMPLE.trim());
        assert_eq!(30, result);
    }
    #[test]
    fn part2_result() {
        let input = utils::resource("src/day04.txt");
        let result = part2(&input);

        assert_eq!(5667240, result);
    }
}
