use std::collections::HashMap;
type Int = usize;
enum Module {
    Broadcaster,
    FlipFlop,
    Conjunction,
}

type System = HashMap<String, (Module, Vec<String>)>;
fn parse_system(input: &str) -> System {
    let mut result = System::new();
    for line in input.trim().split('\n') {
        let split = line.split(" -> ").collect::<Vec<&str>>();
        let into = split[1].split(", ").map(|x| x.to_string()).collect();
        let from = split[0].to_string();
        let (name, module) = match from.as_bytes()[0] {
            b'b' => (from, (Module::Broadcaster, into)),
            b'%' => (from[1..].to_string(), (Module::FlipFlop, into)),
            b'&' => (from[1..].to_string(), (Module::Conjunction, into)),
            x => panic!("unknown module type '{}'", x as char),
        };
        result.insert(name, module);
    }
    return result;
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Signal {
    High,
    Low,
}

fn push_button(
    system: &System,
    states_flip_flop: &mut HashMap<String, bool>,
    states_conjunction: &mut HashMap<String, HashMap<String, Signal>>,
) -> (Int, Int) {
    let mut highs = 0;
    let mut lows = 0;
    let mut work = vec![("button", Signal::Low, "broadcaster")];
    while let Some((source, signal, receiver)) = work.pop() {
        // println!("{} -{:#?}- -> {}", source, signal, receiver);
        match signal {
            Signal::High => highs += 1,
            Signal::Low => lows += 1,
        }
        if let Some((module_type, targets)) = system.get(receiver) {
            let mut next_signal = None;

            match module_type {
                Module::Broadcaster => {
                    next_signal = Some(signal);
                }
                Module::FlipFlop => match signal {
                    Signal::High => {}
                    Signal::Low => {
                        if states_flip_flop[receiver] {
                            states_flip_flop.insert(receiver.to_string(), false);
                            next_signal = Some(Signal::Low);
                        } else {
                            states_flip_flop.insert(receiver.to_string(), true);
                            next_signal = Some(Signal::High);
                        }
                    }
                },
                Module::Conjunction => {
                    states_conjunction
                        .entry(receiver.to_string())
                        .and_modify(|module_state| {
                            module_state.insert(source.to_string(), signal);
                        });
                    let all_high = states_conjunction[receiver]
                        .values()
                        .all(|&s| s == Signal::High);
                    if all_high {
                        next_signal = Some(Signal::Low);
                    } else {
                        next_signal = Some(Signal::High);
                    }
                }
            }

            if let Some(to_send) = next_signal {
                for target in targets {
                    work.insert(0, (receiver, to_send, target));
                }
            }
        }
    }

    (highs, lows)
}

fn part1(input: &str, iterations: i32) -> Int {
    let system = &parse_system(input);
    let mut highs = 0;
    let mut lows = 0;

    // init memory
    let mut states_flip_flop = HashMap::<String, bool>::new();
    let mut states_conjunction = HashMap::<String, HashMap<String, Signal>>::new();
    for (source, (_, targets)) in system {
        for output in targets {
            match system.get(output) {
                Some((Module::Conjunction, _)) => {
                    states_conjunction
                        .entry(output.clone())
                        .or_insert(HashMap::new())
                        .insert(source.clone(), Signal::Low);
                }
                Some((Module::FlipFlop, _)) => {
                    states_flip_flop.insert(output.clone(), false);
                }
                _ => {}
            }
        }
    }

    // push button
    for _ in 0..iterations {
        let (h, l) = push_button(&system, &mut states_flip_flop, &mut states_conjunction);
        highs += h;
        lows += l;
    }
    highs * lows
}

fn part2(input: &str) -> Int {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn part1_example1() {
        let result = part1(
            r#"
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
"#,
            1,
        );
        assert_eq!(result, 4 * 8)
    }
    #[test]
    fn part1_example2() {
        let result = part1(
            r#"
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
"#,
            1000,
        );
        assert_eq!(result, 11687500)
    }

    #[test]
    fn part1_result() {
        let input = utils::resource("src/day20.txt");
        let result = part1(&input, 1000);
        assert_eq!(result, 814934624);
    }

    #[test]
    fn part2_example() {
        let result = part2("");
        assert_eq!(result, 0)
    }

    #[test]
    fn part2_result() {
        let input = utils::resource("src/day20.txt");
        let result = part2(&input);
        assert_eq!(result, 0);
    }
}
