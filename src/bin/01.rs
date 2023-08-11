pub fn part_one(input: &str) -> Option<i32> {
    Some(input.chars().fold(0, |acc, c| match c {
        '(' => acc + 1,
        ')' => acc - 1,
        _ => acc,
    }))
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut floor = 0;
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => {
                floor += 1;
            }
            ')' => {
                floor -= 1;
            }
            _ => (),
        }

        if floor < 0 {
            return Some(i as i32 + 1);
        }
    }

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("(())"), Some(0));
        assert_eq!(part_one("()()"), Some(0));

        assert_eq!(part_one("((("), Some(3));
        assert_eq!(part_one("(()(()("), Some(3));
        assert_eq!(part_one("))((((("), Some(3));

        assert_eq!(part_one("())"), Some(-1));
        assert_eq!(part_one("))("), Some(-1));

        assert_eq!(part_one(")))"), Some(-3));
        assert_eq!(part_one(")())())"), Some(-3));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(")"), Some(1));
        assert_eq!(part_two("()())"), Some(5));
    }
}
