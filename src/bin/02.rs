use std::str::FromStr;

struct PresentBox {
    length: u32,
    width: u32,
    height: u32,
}

impl PresentBox {
    fn calculate_paper_required(&self) -> u32 {
        let sides = [
            (self.length * self.width),
            (self.width * self.height),
            (self.height * self.length),
        ];
        let area = sides.iter().sum::<u32>() * 2;
        let smallest_side = *sides.iter().min().unwrap();
        area + smallest_side
    }

    fn calculate_ribbon_required(&self) -> u32 {
        let mut sides = [self.length, self.width, self.height];
        sides.sort();
        let wrapping_ribbon = sides.iter().take(2).map(|side| *side * 2).sum::<u32>();
        let bow_ribbon = sides.iter().product::<u32>();
        wrapping_ribbon + bow_ribbon
    }
}

impl FromStr for PresentBox {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values = s.split('x').map(|value| value.parse::<u32>().unwrap());
        Ok(Self {
            length: values.next().unwrap(),
            width: values.next().unwrap(),
            height: values.next().unwrap(),
        })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                line.parse::<PresentBox>()
                    .unwrap()
                    .calculate_paper_required()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                line.parse::<PresentBox>()
                    .unwrap()
                    .calculate_ribbon_required()
            })
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("2x3x4"), Some(58));
        assert_eq!(part_one("1x1x10"), Some(43));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("2x3x4"), Some(34));
        assert_eq!(part_two("1x1x10"), Some(14));
    }
}
