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

fn push_button<T>(
    system: &System,
    states_flip_flop: &mut HashMap<String, bool>,
    states_conjunction: &mut HashMap<String, HashMap<String, Signal>>,
    handle_signal: &mut T,
) where
    T: FnMut(&str, Signal, &str),
{
    let mut work = vec![("button", Signal::Low, "broadcaster")];
    while let Some((source, signal, receiver)) = work.pop() {
        handle_signal(source, signal, receiver);
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
}

fn init_memory(
    system: &System,
) -> (
    HashMap<String, bool>,
    HashMap<String, HashMap<String, Signal>>,
) {
    // init memory
    let mut states_flip_flop = HashMap::new();
    let mut states_conjunction = HashMap::new();
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
    (states_flip_flop, states_conjunction)
}

fn part1(input: &str, iterations: i32) -> Int {
    let system = &parse_system(input);
    let mut highs = 0;
    let mut lows = 0;
    let (mut states_flip_flop, mut states_conjunction) = init_memory(system);
    // push button
    for _ in 0..iterations {
        push_button(
            &system,
            &mut states_flip_flop,
            &mut states_conjunction,
            &mut |_, signal, _| match signal {
                Signal::High => {
                    highs += 1;
                }
                Signal::Low => {
                    lows += 1;
                } // println!("{} -{:#?}- -> {}", source, signal, receiver);
            },
        );
    }
    highs * lows
}

fn part2(input: &str) -> Int {
    let system = &parse_system(input);
    let (ref mut states_flip_flop, ref mut states_conjunction) = init_memory(system);

    let rx_source = system
        .iter()
        .filter_map(|(key, (_, targets))| {
            if targets.len() == 1 && targets[0] == "rx" {
                Some(key.clone())
            } else {
                None
            }
        })
        .collect::<Vec<String>>();

    // find out how many times we need to press the button to get a high signal into each
    // of inputs flowing into th the rx_proxy conjunction that sends the low signal to rx
    let rx_proxy_name = &rx_source[0].to_string();
    let mut rx_proxy_inputs = HashMap::<String, Int>::new();
    for (key, _) in &states_conjunction[rx_proxy_name] {
        rx_proxy_inputs.insert(key.clone(), 0);
    }

    let mut cnt = 0;
    loop {
        if rx_proxy_inputs.values().all(|&x| x > 0) {
            // luckily those are all primes...
            return rx_proxy_inputs.values().product();
        }
        cnt += 1;
        push_button(
            system,
            states_flip_flop,
            states_conjunction,
            &mut |source, signal, _| {
                if rx_proxy_inputs.contains_key(source) && signal == Signal::High {
                    rx_proxy_inputs.insert(source.to_string(), cnt);
                }
            },
        );
    }
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
    fn part2_result() {
        let input = utils::resource("src/day20.txt");
        let result = part2(&input);
        assert_eq!(result, 228282646835717);
    }
}
