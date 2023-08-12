use std::str::FromStr;

#[derive(Debug)]
enum Instruction {
    TurnOn((usize, usize), (usize, usize)),
    Toggle((usize, usize), (usize, usize)),
    TurnOff((usize, usize), (usize, usize)),
}

impl Instruction {
    fn execute(&self, grid: &mut [[u8; 1000]; 1000], bit: bool) {
        let (&(start_x, start_y), &(finish_x, finish_y)) = match self {
            Instruction::TurnOn(start, finish) => (start, finish),
            Instruction::Toggle(start, finish) => (start, finish),
            Instruction::TurnOff(start, finish) => (start, finish),
        };

        for row in grid.iter_mut().skip(start_y).take((finish_y + 1) - start_y) {
            for light in row.iter_mut().skip(start_x).take((finish_x + 1) - start_x) {
                match self {
                    Instruction::TurnOn(_, _) => {
                        if bit {
                            *light = 1;
                        } else {
                            *light += 1;
                        }
                    }
                    Instruction::Toggle(_, _) => {
                        if bit {
                            *light = if *light == 0 { 1 } else { 0 };
                        } else {
                            *light += 2;
                        }
                    }
                    Instruction::TurnOff(_, _) => {
                        if bit {
                            *light = 0;
                        } else {
                            *light = light.checked_sub(1).unwrap_or(0);
                        }
                    }
                }
            }
        }
    }
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(' ').collect::<Vec<_>>()[..] {
            ["turn", "on", x, "through", y] => {
                Ok(Self::TurnOn(parse_coordinate(x), parse_coordinate(y)))
            }
            ["toggle", x, "through", y] => {
                Ok(Self::Toggle(parse_coordinate(x), parse_coordinate(y)))
            }
            ["turn", "off", x, "through", y] => {
                Ok(Self::TurnOff(parse_coordinate(x), parse_coordinate(y)))
            }
            _ => Err("Cannot parse, invalid format"),
        }
    }
}

fn parse_coordinate(s: &str) -> (usize, usize) {
    let (x, y) = s.split_once(',').unwrap();
    (x.parse().unwrap(), y.parse().unwrap())
}

pub fn part_one(input: &str) -> Option<usize> {
    let instructions: Vec<Instruction> = input
        .lines()
        .map(|line| line.parse::<Instruction>().unwrap())
        .collect();

    let mut grid = &mut [[0_u8; 1000]; 1000];

    for instruction in instructions {
        instruction.execute(&mut grid, true);
    }

    Some(
        grid.iter()
            .map(|row| row.iter().filter(|&&active| active == 1).count())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let instructions: Vec<Instruction> = input
        .lines()
        .map(|line| line.parse::<Instruction>().unwrap())
        .collect();

    let mut grid = &mut [[0_u8; 1000]; 1000];

    for instruction in instructions {
        instruction.execute(&mut grid, false);
    }

    Some(
        grid.iter()
            .map(|row| row.iter().map(|&light| light as u32).sum::<u32>())
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(998_996));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(1_001_996));

        assert_eq!(part_two("turn on 0,0 through 0,0"), Some(1));

        assert_eq!(part_two("toggle 0,0 through 999,999"), Some(2_000_000));
    }
}
