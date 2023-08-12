use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy)]
enum Signal<'a> {
    Value(u16),
    Wire(&'a str),
}

impl<'a> Signal<'a> {
    fn new(s: &'a str) -> Self {
        match s.parse::<u16>() {
            Ok(signal) => Signal::Value(signal),
            Err(_) => Signal::Wire(s),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Gate<'a> {
    Signal {
        wire: &'a str,
        signal: Signal<'a>,
    },
    And {
        wire: &'a str,
        left: Signal<'a>,
        right: Signal<'a>,
    },
    Or {
        wire: &'a str,
        left: Signal<'a>,
        right: Signal<'a>,
    },
    LShift {
        wire: &'a str,
        other: &'a str,
        value: u16,
    },
    RShift {
        wire: &'a str,
        other: &'a str,
        value: u16,
    },
    Not {
        wire: &'a str,
        other: &'a str,
    },
}

impl<'a> Gate<'a> {
    fn new(s: &'a str) -> Self {
        match s.split_once(" -> ").expect("Invalid format") {
            (left, wire) if left.contains("AND") => {
                let mut iter = left.split_whitespace();
                Self::And {
                    wire,
                    left: Signal::new(iter.next().unwrap()),
                    right: Signal::new(iter.nth(1).unwrap()),
                }
            }
            (left, wire) if left.contains("OR") => {
                let mut iter = left.split_whitespace();
                Self::Or {
                    wire,
                    left: Signal::new(iter.next().unwrap()),
                    right: Signal::new(iter.nth(1).unwrap()),
                }
            }
            (left, wire) if left.contains("LSHIFT") => {
                let mut iter = left.split_whitespace();
                Self::LShift {
                    wire,
                    other: iter.next().unwrap(),
                    value: iter.nth(1).unwrap().parse().unwrap(),
                }
            }
            (left, wire) if left.contains("RSHIFT") => {
                let mut iter = left.split_whitespace();
                Self::RShift {
                    wire,
                    other: iter.next().unwrap(),
                    value: iter.nth(1).unwrap().parse().unwrap(),
                }
            }
            (left, wire) if left.contains("NOT") => Self::Not {
                wire,
                other: left.split_whitespace().nth(1).unwrap(),
            },
            (signal, wire) => Self::Signal {
                wire,
                signal: Signal::new(signal),
            },
        }
    }
}

fn simulate_circuit<'a>(wires: &mut BTreeMap<&'a str, u16>, gates: &'a Vec<Gate>) {
    for gate in gates {
        match gate {
            Gate::Signal { wire, signal } => match signal {
                Signal::Value(signal) => {
                    wires.insert(wire, *signal);
                }
                Signal::Wire(w) => {
                    if let Some(signal) = wires.get(w) {
                        wires.insert(wire, *signal);
                    };
                }
            },
            Gate::And { wire, left, right } => {
                let left = match left {
                    Signal::Value(signal) => Some(signal),
                    Signal::Wire(w) => wires.get(w),
                };
                let right = match right {
                    Signal::Value(signal) => Some(signal),
                    Signal::Wire(w) => wires.get(w),
                };

                if let Some(left) = left {
                    if let Some(right) = right {
                        wires.insert(wire, left & right);
                    };
                };
            }
            Gate::Or { wire, left, right } => {
                let left = match left {
                    Signal::Value(signal) => Some(signal),
                    Signal::Wire(w) => wires.get(w),
                };
                let right = match right {
                    Signal::Value(signal) => Some(signal),
                    Signal::Wire(w) => wires.get(w),
                };

                if let Some(left) = left {
                    if let Some(right) = right {
                        wires.insert(wire, left | right);
                    };
                };
            }
            Gate::LShift { wire, other, value } => {
                if let Some(other) = wires.get(other) {
                    wires.insert(wire, other << value);
                };
            }
            Gate::RShift { wire, other, value } => {
                if let Some(other) = wires.get(other) {
                    wires.insert(wire, other >> value);
                };
            }
            Gate::Not { wire, other } => {
                if let Some(other) = wires.get(other) {
                    wires.insert(wire, !other);
                };
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u16> {
    let mut wires: BTreeMap<&str, u16> = BTreeMap::new();

    let gates: Vec<Gate> = input.lines().map(Gate::new).collect();

    while wires.len() != gates.len() {
        simulate_circuit(&mut wires, &gates);
    }

    wires.get("a").copied()
}

pub fn part_two(input: &str) -> Option<u16> {
    let mut wires: BTreeMap<&str, u16> = BTreeMap::new();

    let gates: Vec<Gate> = input.lines().map(Gate::new).collect();

    while wires.len() != gates.len() {
        simulate_circuit(&mut wires, &gates);
    }

    let gates: Vec<Gate> = gates
        .iter()
        .map(|gate| match gate {
            Gate::Signal { wire, .. } => {
                if *wire == "b" {
                    Gate::Signal {
                        wire,
                        signal: Signal::Value(*wires.get("a").unwrap()),
                    }
                } else {
                    *gate
                }
            }
            _ => *gate,
        })
        .collect();
    wires.clear();

    while wires.len() != gates.len() {
        simulate_circuit(&mut wires, &gates);
    }

    wires.get("a").copied()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(65079));
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}
