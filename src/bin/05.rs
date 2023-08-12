use std::collections::HashMap;

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .filter(|line| {
                let vowels = line.chars().filter(|c| VOWELS.contains(c)).count();
                if vowels < 3 {
                    return false;
                }
                let mut double = false;
                for window in line.chars().collect::<Vec<_>>().windows(2) {
                    match window {
                        ['a', 'b'] | ['c', 'd'] | ['p', 'q'] | ['x', 'y'] => return false,
                        [a, b] if a == b => {
                            double = true;
                        }
                        _ => (),
                    }
                }
                double
            })
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .filter(|line| {
                let line = line.chars().collect::<Vec<_>>();

                let mut map = HashMap::new();
                for (i, window) in line.windows(2).enumerate() {
                    match window {
                        [a, b] => {
                            map.entry((*a, *b))
                                .and_modify(|curr: &mut Vec<usize>| curr.push(i))
                                .or_insert(vec![i]);
                        }
                        _ => todo!(),
                    }
                }
                let contains_pattern = map
                    .iter()
                    .filter_map(|(_, v)| if v.len() > 1 { Some(v) } else { None })
                    .any(|v| {
                        let first = v.first().unwrap();
                        v.iter().any(|i| i - first > 1)
                    });

                let repeat_surrounding = line
                    .windows(3)
                    .any(|window| matches!(window, [a, _, b] if a == b));

                contains_pattern && repeat_surrounding
            })
            .count(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = "qjhvhtzxzqqjkmpb\nxxyxx\nuurcxstgmygtbstg\nieodomkazucvgmuy";
        assert_eq!(part_two(input), Some(2));

        let input = "xxxddetvrlpzsfpq";
        assert_eq!(part_two(input), Some(0));
    }
}
