use serde_json::Value;

fn sum_of_numbers(value: &Value) -> i32 {
    match value {
        Value::Number(n) => n.as_i64().unwrap() as i32,
        Value::Array(a) => a.iter().map(sum_of_numbers).sum(),
        Value::Object(o) => o.values().map(sum_of_numbers).sum(),
        _ => 0,
    }
}

fn sum_of_numbers_minus_red(value: &Value) -> i32 {
    match value {
        Value::Number(n) => n.as_i64().unwrap() as i32,
        Value::Array(a) => a.iter().map(sum_of_numbers_minus_red).sum(),
        Value::Object(o) => {
            let red = o
                .values()
                .find(|value| match value {
                    Value::String(s) => s == "red",
                    _ => false,
                })
                .is_some();

            if red {
                return 0;
            }

            o.values().map(sum_of_numbers_minus_red).sum()
        }
        _ => 0,
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let value = serde_json::from_str::<Value>(input).unwrap();

    Some(sum_of_numbers(&value))
}

pub fn part_two(input: &str) -> Option<i32> {
    let value = serde_json::from_str::<Value>(input).unwrap();

    Some(sum_of_numbers_minus_red(&value))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(18));
    }

    #[test]
    fn test_part_two() {
        let input =
            r#"[[1,2,3], [1,{"c":"red","b":2},3], {"d":"red","e":[1,2,3,4],"f":5}, [1,"red",5]]"#;
        assert_eq!(part_two(input), Some(16));
    }
}
