use std::collections::BTreeSet;

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => Self::North,
            'v' => Self::South,
            '>' => Self::East,
            '<' => Self::West,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Default)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn move_to(&mut self, direction: &Direction) {
        match direction {
            Direction::North => self.y += 1,
            Direction::South => self.y -= 1,
            Direction::East => self.x += 1,
            Direction::West => self.x -= 1,
        }
    }

    fn as_tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let moves: Vec<Direction> = input.chars().map(Into::into).collect();
    let mut position = Position::default();
    let mut visited = BTreeSet::new();
    visited.insert(position.as_tuple());

    for direction in moves {
        position.move_to(&direction);
        visited.insert(position.as_tuple());
    }

    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let moves: Vec<Direction> = input.chars().map(Into::into).collect();
    let mut santa = Position::default();
    let mut robo_santa = Position::default();
    let mut visited = BTreeSet::new();
    visited.insert(santa.as_tuple());

    for (i, direction) in moves.iter().enumerate() {
        let santa = if i % 2 == 0 {
            &mut santa
        } else {
            &mut robo_santa
        };
        santa.move_to(direction);
        visited.insert(santa.as_tuple());
    }

    Some(visited.len() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(">"), Some(2));
        assert_eq!(part_one("^>v<"), Some(4));
        assert_eq!(part_one("^v^v^v^v^v"), Some(2));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("^v"), Some(3));
        assert_eq!(part_two("^>v<"), Some(3));
        assert_eq!(part_two("^v^v^v^v^v"), Some(11));
    }
}
