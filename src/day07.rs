use std::collections::HashMap;

type Int = u128;
type Card = u8;
const SHIFT: Int = 8; // 8 from u8 in Card
type Hand = Vec<char>;

const fn card_power(ch: char) -> Card {
    match ch {
        'A' => 13,
        'K' => 12,
        'Q' => 11,
        'J' => 10,
        'T' => 9,
        '9' => 8,
        '8' => 7,
        '7' => 6,
        '6' => 5,
        '5' => 4,
        '4' => 3,
        '3' => 2,
        '2' => 1,
        _ => panic!("unexpected input"),
    }
}

const ORDER_1: &str = "23456789TJQKA";
const ORDER_2: &str = "J23456789TQKA";
const JOKER: &char = &'J';
const NO_JOKER: &char = &'_';
fn hand_power(hand: &str, order: &'static str, joker: &'static char) -> Int {
    let mut freq = HashMap::<char, u8>::new();
    for ch in hand.chars() {
        let existing = freq.remove(&ch);
        freq.insert(ch, existing.unwrap_or_default() + 1);
    }

    // resolve jokers
    match freq.remove(joker) {
        // we've got 5 of a kind, keep it
        Some(5) => {
            freq.insert(*joker, 5);
        }
        // add it to the one of the card types with the highest frequency, doesn't matter which
        Some(joker_count) => {
            let (&max_card, &max_freq) = freq.iter().max_by_key(|(_, &cnt)| cnt).unwrap();
            freq.remove(&max_card);
            freq.insert(max_card, max_freq + joker_count);
        }
        None => {}
    }

    let freq: HashMap<char, u8> = freq;

    // upper bits are the type of the hand, followed by cards in order
    // e.g. if we only had 3 cards in the hand, and 4 types of cards (A=1, B=2, C=4, D=8)
    // we'd ancode them a has 3, has 2, single cards in order
    //                  b_____0______0_00000000_00000000_00000000
    // AAA   would be b10_00000001_00000001_00000001
    // DDC   would be b01_00001000_00001000_00000100

    let upper_type_bits: Int = freq
        .iter()
        .filter(|(_, &count)| count > 2)
        .map(|(_, &count)| 1 << (count - 1))
        .sum();
    let number_of_pairs = freq.iter().filter(|(_, &count)| count == 2).count() as Int; // 1 -> b01 or 2 -> b10
    let upper_type_bits = (upper_type_bits << 2) + number_of_pairs;
    // we get b10000 => 5 of a kind
    //        b01000 => 4 of a kind
    //        b00101 => full house
    //        b00100 => 3 of a kind
    //        b00010 => 2 pairs
    //        b00001 => 1 pair
    //        b00000 => high card

    // now add the remainder of cards in order, so the last card is the lowest byte
    hand.chars()
        .map(|ch| order.find(ch).expect("unexpected character") as u8)
        .fold(upper_type_bits, |agg, card| (agg << SHIFT) + (card as Int))
}

fn solve(input: &str, order: &'static str, joker: &'static char) -> Int {
    let mut hands: Vec<(Int, Int, &str)> = input
        .trim()
        .split('\n')
        .map(|line| {
            let (hand_str, bet_str) = line.split_at(5);
            let bet = bet_str.trim().parse::<Int>().expect("all bets are off");
            return (hand_power(hand_str.trim(), order, joker), bet, hand_str);
        })
        .collect();
    hands.sort_by(|(a, _, _), (b, _, _)| PartialOrd::partial_cmp(a, b).unwrap());

    hands
        .iter()
        .enumerate()
        .map(|(zrank, (_, bet, _))| (zrank as Int + 1) * bet)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;

    const EXAMPLE: &str = r#"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

    #[test]
    fn part1_example() {
        let result = solve(EXAMPLE.trim(), ORDER_1, NO_JOKER);

        assert_eq!(6440, result);
    }

    #[test]
    fn part1_result() {
        let input = resource("src/day07.txt");
        let result = solve(input.trim(), ORDER_1, NO_JOKER);

        assert_eq!(250120186, result);
    }

    #[test]
    fn part2_example() {
        let result = solve(EXAMPLE.trim(), ORDER_2, JOKER);

        assert_eq!(5905, result);
    }

    #[test]
    fn part2_result() {
        let input = resource("src/day07.txt");
        let result = solve(input.trim(), ORDER_2, JOKER);

        assert_eq!(250665248, result);
    }
}
