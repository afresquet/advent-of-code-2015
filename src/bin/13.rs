use std::collections::{BTreeMap, BTreeSet};

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<i32> {
    let mut people: BTreeMap<&str, BTreeMap<&str, i32>> = input
        .lines()
        .map(|line| line.split_whitespace().next().unwrap())
        .collect::<BTreeSet<&str>>()
        .into_iter()
        .map(|name| (name, BTreeMap::new()))
        .collect();

    for line in input.lines() {
        let mut iter = line.split_whitespace();
        let person = people.entry(iter.next().unwrap());
        let multiplier = iter.nth(1).unwrap();
        let multiplier = if multiplier == "gain" { 1 } else { -1 };
        let happiness = iter.next().unwrap().parse::<i32>().unwrap() * multiplier;
        let relative = iter.nth(6).map(|name| &name[..(name.len() - 1)]).unwrap();
        person.and_modify(|v: &mut BTreeMap<&str, i32>| {
            v.insert(relative, happiness);
        });
    }

    people
        .iter()
        .permutations(people.len())
        .map(|perm| {
            perm.iter()
                .cycle()
                .take(perm.len() + 2)
                .tuple_windows()
                .map(|(left, person, right)| {
                    let left = person.1.get(left.0).unwrap();
                    let right = person.1.get(right.0).unwrap();
                    left + right
                })
                .sum()
        })
        .max()
}

pub fn part_two(input: &str) -> Option<i32> {
    let people_names: BTreeSet<&str> = input
        .lines()
        .map(|line| line.split_whitespace().next().unwrap())
        .collect();
    let mut people: BTreeMap<&str, BTreeMap<&str, i32>> = people_names
        .clone()
        .into_iter()
        .map(|name| {
            let mut map = BTreeMap::new();
            map.insert("Myself", 0);
            (name, map)
        })
        .collect();
    let myself = {
        let mut map = BTreeMap::new();
        for name in people_names {
            map.insert(name, 0);
        }
        map
    };
    people.insert("Myself", myself);

    for line in input.lines() {
        let mut iter = line.split_whitespace();
        let person = people.entry(iter.next().unwrap());
        let multiplier = iter.nth(1).unwrap();
        let multiplier = if multiplier == "gain" { 1 } else { -1 };
        let happiness = iter.next().unwrap().parse::<i32>().unwrap() * multiplier;
        let relative = iter.nth(6).map(|name| &name[..(name.len() - 1)]).unwrap();
        person.and_modify(|v: &mut BTreeMap<&str, i32>| {
            v.insert(relative, happiness);
        });
    }

    people
        .iter()
        .permutations(people.len())
        .map(|perm| {
            perm.iter()
                .cycle()
                .take(perm.len() + 2)
                .tuple_windows()
                .map(|(left, person, right)| {
                    let left = person.1.get(left.0).unwrap();
                    let right = person.1.get(right.0).unwrap();
                    left + right
                })
                .sum()
        })
        .max()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(330));
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), None);
    }
}
