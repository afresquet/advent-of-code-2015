pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let length = line.len() as u32;
                let line = &mut line[1..(line.len() - 1)].chars();
                let mut characters: u32 = 0;
                while let Some(c) = line.next() {
                    match c {
                        '\\' => match line.next() {
                            Some('\\') | Some('"') => {
                                characters += 1;
                            }
                            Some('x') => {
                                line.next();
                                line.next();

                                characters += 1;
                            }
                            _ => unreachable!(),
                        },
                        _ => {
                            characters += 1;
                        }
                    }
                }
                length - characters
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let original_characters = line.len() as u32;
                let line = &mut line[1..(line.len() - 1)].chars();
                let mut characters: u32 = 6;
                while let Some(c) = line.next() {
                    match c {
                        '\\' => match line.next() {
                            Some('\\') | Some('"') => {
                                characters += 4;
                            }
                            Some('x') => {
                                line.next();
                                line.next();

                                characters += 5;
                            }
                            _ => unreachable!(),
                        },
                        _ => {
                            characters += 1;
                        }
                    }
                }

                characters - original_characters
            })
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(12));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(19));
    }
}
