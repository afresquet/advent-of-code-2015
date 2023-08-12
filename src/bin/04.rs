// Tests pass but answer is wrong somehow ¯\_(ツ)_/¯

fn find_nonce_for_suffix(secret_key: &str, prefix: &'static str) -> Option<u32> {
    for nonce in 1.. {
        let digest = md5::compute(format!("{secret_key}{nonce}"));
        if format!("{digest:x}").starts_with(prefix) {
            return Some(nonce);
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<u32> {
    find_nonce_for_suffix(input, "00000")
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_part_one() {
        assert_eq!(part_one("abcdef"), Some(609043));
        assert_eq!(part_one("pqrstuv"), Some(1048970));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), None);
    }
}
