use std::str::FromStr;

#[derive(Debug)]
enum State {
    Flying,
    Resting,
}

#[derive(Debug)]
struct Reindeer {
    flying_speed: u32,
    flying_time: u32,
    resting_time: u32,
    state: State,
    distance: u32,
    countdown: u32,
    score: u32,
}

impl Reindeer {
    fn step_in_simulation(&mut self) {
        match self.state {
            State::Flying => {
                self.distance += self.flying_speed;
                self.countdown -= 1;
                if self.countdown == 0 {
                    self.state = State::Resting;
                    self.countdown = self.resting_time;
                }
            }
            State::Resting => {
                self.countdown -= 1;
                if self.countdown == 0 {
                    self.state = State::Flying;
                    self.countdown = self.flying_time;
                }
            }
        }
    }
}

impl FromStr for Reindeer {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        let flying_speed = iter.nth(3).unwrap().parse().unwrap();
        let flying_time = iter.nth(2).unwrap().parse().unwrap();
        let resting_time = iter.nth(6).unwrap().parse().unwrap();

        Ok(Self {
            flying_speed,
            flying_time,
            resting_time,
            state: State::Flying,
            distance: 0,
            countdown: flying_time,
            score: 0,
        })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut reindeers: Vec<Reindeer> = input.lines().map(|line| line.parse().unwrap()).collect();

    for _ in 0..2503 {
        for reindeer in reindeers.iter_mut() {
            reindeer.step_in_simulation();
        }
    }

    reindeers.sort_by_key(|reindeer| reindeer.distance);

    reindeers.last().map(|reindeer| reindeer.distance)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut reindeers: Vec<Reindeer> = input.lines().map(|line| line.parse().unwrap()).collect();

    for _ in 0..2503 {
        for reindeer in reindeers.iter_mut() {
            reindeer.step_in_simulation();
        }

        reindeers.sort_by_key(|reindeer| reindeer.distance);

        let winning_distance = reindeers.last().map(|reindeer| reindeer.distance).unwrap();

        for reindeer in reindeers
            .iter_mut()
            .filter(|reindeer| reindeer.distance == winning_distance)
        {
            reindeer.score += 1;
        }
    }

    reindeers.sort_by_key(|reindeer| reindeer.score);

    reindeers.last().map(|reindeer| reindeer.score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(2660));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(1564));
    }
}
