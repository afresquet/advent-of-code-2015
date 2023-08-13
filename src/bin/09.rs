use itertools::Itertools;
use std::collections::BTreeSet;

struct Distance<'a> {
    from: &'a str,
    to: &'a str,
    distance: u32,
}

impl<'a> Distance<'a> {
    fn new(s: &'a str) -> Self {
        let (locations, distance) = s.split_once(" = ").unwrap();
        let (from, to) = locations.split_once(" to ").unwrap();
        Self {
            from,
            to,
            distance: distance.parse().unwrap(),
        }
    }
}

fn calculate_distances(distances: &[Distance]) -> Vec<u32> {
    let cities: BTreeSet<&str> = distances.iter().flat_map(|d| vec![d.from, d.to]).collect();
    let perms = cities.iter().permutations(cities.len()).collect::<Vec<_>>();
    perms
        .iter()
        .map(|perm| {
            perm.iter().tuple_windows().fold(0, |acc, (&&from, &&to)| {
                let d = distances
                    .iter()
                    .find(|d| (d.from == from || d.from == to) && (d.to == from || d.to == to))
                    .unwrap();
                acc + d.distance
            })
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let distances: Vec<Distance> = input.lines().map(Distance::new).collect();

    let distances = calculate_distances(&distances);

    distances.into_iter().min()
}

pub fn part_two(input: &str) -> Option<u32> {
    let distances: Vec<Distance> = input.lines().map(Distance::new).collect();

    let distances = calculate_distances(&distances);

    distances.into_iter().max()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(982));
    }
}
