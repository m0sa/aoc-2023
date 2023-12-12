type Int = u64;
type Record = (Vec<char>, Vec<usize>);
type Cache = std::collections::HashMap<String, Int>;

fn parse_line(line: &str, expand: Int) -> Record {
    let split: Vec<&str> = line.split(' ').collect();
    let mut springs: String = split[0].chars().collect();
    let mut groups: String = split[1].chars().collect();
    for _ in 0..(expand - 1) {
        springs.push('?');
        springs.push_str(split[0]);
        groups.push(',');
        groups.push_str(split[1]);
    }
    return (
        springs.chars().collect(),
        groups
            .split(',')
            .map(|x| x.parse::<usize>().expect("not a int"))
            .collect(),
    );
}

fn solve_rec(spring: &Vec<char>, groups: &Vec<usize>, cache: &mut Cache) -> Int {
    let mut key: String = "".to_string();
    key.push_str(spring.iter().map(|&x| x).collect::<String>().as_str());
    key.push(' ');
    key.push_str(
        groups
            .iter()
            .map(|&x| format!("{},", x))
            .collect::<String>()
            .as_str(),
    );

    return match cache.get(&key) {
        Some(&x) => x,
        None => {
            let result = count_rec(spring, groups, cache);
            cache.insert(key, result);
            result
        }
    };
}

fn count_rec(springs: &Vec<char>, groups: &Vec<usize>, cache: &mut Cache) -> Int {
    match (groups.first(), springs.first()) {
        (None, _) => match springs.iter().find(|&c| *c == '#') {
            None => 1,
            Some(_) => 0,
        },
        (_, None) => 0, // no more springs to match, although we still have groups to match
        (_, Some('.')) => {
            let dots_trimmed = springs
                .iter()
                .skip_while(|&c| *c == '.')
                .map(|&c| c)
                .collect();
            solve_rec(&dots_trimmed, groups, cache)
        }
        (_, Some('?')) => {
            let mut right = springs.clone();
            right[0] = '.';
            let mut left = springs.clone();
            left[0] = '#';
            solve_rec(&left, &groups, cache) + solve_rec(&right, &groups, cache)
        }
        (Some(0), Some('#')) => 0,
        (Some(&group_size), Some('#')) => {
            if springs.len() < group_size {
                return 0;
            }

            if springs
                .iter()
                .take(group_size)
                .find(|&c| *c == '.')
                .is_some()
            {
                return 0;
            }

            let groups_remaining: Vec<usize> = groups[1..].into();
            if groups_remaining.len() > 0 {
                // if the group is followed by a '#' the counts are off
                if springs.len() < group_size + 1 || springs[group_size] == '#' {
                    return 0;
                }
                // it needs to be followed by a '?' or '.'
                // we can skip that character, too
                let springs = springs[group_size + 1..].into();
                solve_rec(&springs, &groups_remaining, cache)
            } else {
                let tail = &springs[group_size..].into();
                solve_rec(&tail, &groups_remaining, cache)
            }
        }
        _ => panic!("Invalid input"),
    }
}

fn part1(input: &str) -> Int {
    let mut cache = Cache::new();
    input
        .trim()
        .split('\n')
        .map(|ln| parse_line(ln, 1))
        .map(|(chars, counts)| solve_rec(&chars, &counts, &mut cache))
        .sum()
}

fn part2(input: &str) -> Int {
    let mut cache = Cache::new();
    input
        .trim()
        .split('\n')
        .map(|ln| parse_line(ln, 5))
        .map(|(chars, counts)| solve_rec(&chars, &counts, &mut cache))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;
    const EXAMPLE: &str = r#"
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
        "#;
    #[test]
    fn part1_example() {
        let result = part1(EXAMPLE);
        assert_eq!(21, result);
    }

    #[test]
    fn part1_result() {
        let result = part1(&resource("src/day12.txt"));
        assert_eq!(7771, result);
    }

    #[test]
    fn part2_example() {
        let result = part2(EXAMPLE.trim());
        assert_eq!(525152, result);
    }

    #[test]
    fn part2_result() {
        let result = part2(&resource("src/day12.txt"));
        assert_eq!(10861030975833, result);
    }
}
