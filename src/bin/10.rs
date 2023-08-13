use itertools::Itertools;

fn group_by_chars(input: &str) -> Vec<String> {
    input
        .chars()
        .group_by(|c| *c)
        .into_iter()
        .map(|(_, group)| group.collect::<String>())
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut result: String = input.to_string();

    for _ in 0..40 {
        let groups = group_by_chars(&result);

        result = groups
            .iter()
            .map(|group| {
                let length = group.len();
                let character = group.chars().next().unwrap();
                format!("{length}{character}")
            })
            .collect();
    }

    Some(result.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut result: String = input.to_string();

    for _ in 0..50 {
        let groups = group_by_chars(&result);

        result = groups
            .iter()
            .map(|group| {
                let length = group.len();
                let character = group.chars().next().unwrap();
                format!("{length}{character}")
            })
            .collect();
    }

    Some(result.len())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}
