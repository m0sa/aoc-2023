type Int = i64;
type Range = std::ops::Range<Int>;

struct IntMap {
    source: Range,
    destination: Range,
}

impl IntMap {
    fn translate(self: &IntMap, value: &Int) -> Int {
        let offset = value.clone() - self.source.start;
        let destination = self.destination.start + offset;
        return destination;
    }
}

fn parse_mapping(line: &str) -> IntMap {
    let split: Vec<Int> = line.split(' ').map(|x| x.parse().unwrap()).collect();
    IntMap {
        destination: split[0]..(split[0] + split[2]),
        source: split[1]..(split[1] + split[2]),
    }
}

fn parse_layers(lines: &Vec<&str>) -> Vec<Vec<IntMap>> {
    let mut layers = Vec::<Vec<IntMap>>::new();
    let mut builder = Vec::<IntMap>::new();
    for line in lines.iter() {
        if line.is_empty() {
        } else if line.contains("-to-") {
            if builder.len() > 0 {
                let mut layer = Vec::<IntMap>::new();
                layer.append(&mut builder); // this also clears the builder
                layers.push(layer);
            }
        } else {
            builder.push(parse_mapping(line));
        }
    }
    layers.push(builder);
    return layers;
}

fn intersect(a: &Range, b: &Range) -> Option<Range> {
    if a.contains(&b.start) {
        return Some(b.start..std::cmp::min(a.end, b.end));
    }

    if b.contains(&a.start) {
        return Some(a.start..std::cmp::min(a.end, b.end));
    }

    return None;
}

fn solve(seed: &Vec<Range>, layers: &Vec<Vec<IntMap>>) -> Int {
    let results = layers.iter().fold(seed.clone(), |agg, layer| {
        let mut next = Vec::<Range>::new();
        let mut todo = agg.into_iter().collect::<Vec<Range>>();
        while todo.len() > 0 {
            let cur = todo.pop().unwrap();

            let mut was_mapped = false;
            for map in layer {
                let int_opt = intersect(&cur, &map.source);
                if int_opt.is_none() {
                    continue;
                }

                let int = int_opt.unwrap();
                let dest = map.translate(&int.start)..map.translate(&int.end);
                next.push(dest);

                // the range might split in 3 by the intersection, we might still need to process excess at the beginnig and end
                if cur.start < int.start {
                    todo.push(cur.start..int.start);
                }
                if int.end < cur.end {
                    todo.push(int.end..cur.end);
                }
                was_mapped = true;
            }

            if !was_mapped {
                next.push(cur);
            }
        }
        return next;
    });
    return results.into_iter().map(|s| s.start).min().unwrap();
}

fn part1(input: &str) -> Int {
    let lines: Vec<&str> = input.trim().split('\n').collect();
    let seed_ranges: Vec<Range> = lines[0]
        .split(' ')
        .skip(1)
        .map(|x| x.parse().unwrap())
        .map(|x| x..(x + 1))
        .collect();

    let layers = parse_layers(&lines.into_iter().skip(2).collect());
    return solve(&seed_ranges, &layers);
}

fn part2(input: &str) -> Int {
    let lines: Vec<&str> = input.trim().split('\n').collect();
    let seed_numbers: Vec<Int> = lines[0]
        .split(' ')
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();

    let mut seed_ranges = Vec::<Range>::new();
    for i in 0..seed_numbers.len() {
        if i % 2 == 1 {
            continue;
        }
        let start = seed_numbers[i];
        let end = seed_numbers[i] + seed_numbers[i + 1];
        seed_ranges.push(start..end);
    }

    let layers = parse_layers(&lines.into_iter().skip(2).collect());
    return solve(&seed_ranges, &layers);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::resource;
    #[test]
    fn part1_example() {
        let input = resource("src/day05.example.txt");
        let result = part1(&input);

        assert_eq!(35, result);
    }

    #[test]
    fn part1_result() {
        let input = resource("src/day05.txt");
        let result = part1(&input);

        assert_eq!(1181555926, result);
    }

    #[test]
    fn part2_example() {
        let input = resource("src/day05.example.txt");
        let result = part2(&input);

        assert_eq!(46, result);
    }
    #[test]
    fn part2_result() {
        let input = resource("src/day05.txt");
        let result = part2(&input);

        assert_eq!(37806486, result);
    }
}
