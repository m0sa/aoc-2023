use std::collections::HashMap;

fn hash<'a>(chars: std::str::Chars<'a>) -> u8 {
    let mut current: usize = 0;
    for ch in chars {
        current += ch as usize;
        current *= 17;
        current %= 256;
    }
    current as u8
}

fn part1(input: &str) -> u128 {
    input
        .split(',')
        .map(|part| hash(part.chars()) as u128)
        .sum()
}

type LabeledLens = (String, u8);
type LenseBox = Vec<LabeledLens>;
trait LenseBoxContentFinder {
    fn find_lense(self, target: &str) -> Option<usize>;
}
impl LenseBoxContentFinder for &LenseBox {
    fn find_lense(self, target: &str) -> Option<usize> {
        self.iter()
            .enumerate()
            .find_map(|(slot, (lbl, _))| (lbl == target).then_some(slot))
    }
}

fn part2(input: &str) -> u128 {
    let instructions = input.split(',');
    let mut boxes = HashMap::<u8, LenseBox>::new();
    for inst in instructions {
        let op_index = inst.find(&['-', '=']).expect("operation not found");
        let lens_label = &inst[0..op_index];
        let box_id = hash(lens_label.chars());
        match inst.as_bytes()[op_index] {
            b'-' => {
                boxes.entry(box_id).and_modify(|box_content| {
                    match box_content.find_lense(lens_label) {
                        None => {}
                        Some(rmi) => {
                            box_content.remove(rmi);
                        }
                    };
                });
            }
            b'=' => {
                let focal_length = inst[op_index + 1..].parse::<u8>().expect(inst);
                boxes
                    .entry(box_id)
                    .and_modify(|box_content| {
                        // box modification
                        match box_content.find_lense(lens_label) {
                            Some(idx) => {
                                box_content[idx].1 = focal_length;
                            }
                            None => {
                                box_content.push((lens_label.to_string(), focal_length));
                            }
                        };
                    })
                    .or_insert(vec![(lens_label.to_string(), focal_length)]);
            }
            c => panic!("unexpected character {}", c as char),
        };
    }

    let mut result: u128 = 0;
    for (box_id, lensens) in boxes {
        let box_number = box_id as u128 + 1;
        for (slot, (_, focal_length)) in lensens.iter().enumerate() {
            let slot_number = slot as u128 + 1;
            let fl = *focal_length as u128;
            result += box_number * slot_number * fl;
        }
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    const EXAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    #[test]
    fn part1_example() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 1320)
    }

    #[test]
    fn part1_result() {
        let input = utils::resource("src/day15.txt");
        let result = part1(&input);
        assert_eq!(result, 514281);
    }

    #[test]
    fn part2_example() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 145)
    }

    #[test]
    fn part2_result() {
        let input = utils::resource("src/day15.txt");
        let result = part2(&input);
        assert_eq!(result, 244199);
    }
}
