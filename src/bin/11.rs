use std::collections::HashMap;

use itertools::Itertools;

struct Password(Vec<char>);

impl Password {
    fn new(input: &str) -> Self {
        let chars = input.chars().collect();
        Self(chars)
    }

    fn next_password(&mut self) {
        let Self(chars) = self;

        let mut index = chars.len() - 1;

        loop {
            let current = chars[index];
            let next = current as u8 + 1;
            if next <= 'z' as u8 {
                chars[index] = next as char;
                break;
            }
            chars[index] = 'a';
            index -= 1;
        }
    }

    fn is_valid(&self) -> bool {
        let Self(chars) = self;

        let ascending_pattern = chars.iter().tuple_windows().any(|(&x, &y, &z)| {
            let x = x as u32;
            let y = y as u32;
            let z = z as u32;
            x + 1 == y && y + 1 == z
        });

        let valid_letters = chars.iter().all(|&c| c != 'i' && c != 'o' && c != 'l');

        let mut map = HashMap::new();
        for (i, (&x, &y)) in chars.iter().tuple_windows().enumerate() {
            if x == y {
                map.entry((x, y))
                    .and_modify(|v: &mut Vec<usize>| {
                        v.push(i);
                    })
                    .or_insert_with(|| vec![i]);
            }
        }
        let mut pair_indexes = map.values().flatten().collect::<Vec<_>>();
        pair_indexes.sort();
        let contains_two_pairs = match pair_indexes.first() {
            Some(&&first) => pair_indexes.iter().any(|&&i| i - first > 1),
            None => false,
        };

        ascending_pattern && valid_letters && contains_two_pairs
    }
}

impl From<Password> for String {
    fn from(value: Password) -> Self {
        let Password(chars) = value;
        chars.iter().collect()
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut password = Password::new(input);

    password.next_password();
    while !password.is_valid() {
        password.next_password();
    }

    Some(password.into())
}

pub fn part_two(input: &str) -> Option<String> {
    let mut password = Password::new(input);

    for _ in 0..2 {
        password.next_password();
        while !password.is_valid() {
            password.next_password();
        }
    }

    Some(password.into())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some("abcdffaa".to_string()));

        assert_eq!(part_one("ghijklmn"), Some("ghjaabcc".to_string()));
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), None);
    }

    #[test]
    fn password_is_valid() {
        let password = Password::new("abcdffaa");
        assert!(password.is_valid());

        let password = Password::new("ghjaabcc");
        assert!(password.is_valid());

        let password = Password::new("hijklmmn");
        assert!(!password.is_valid());

        let password = Password::new("abbceffg");
        assert!(!password.is_valid());

        let password = Password::new("abbcegjk");
        assert!(!password.is_valid());
    }
}
