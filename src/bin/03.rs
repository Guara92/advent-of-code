use std::sync::LazyLock;

use regex::Regex;

advent_of_code::solution!(3);

static RE_1: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap());
static RE_2: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(mul\((?<op1>[0-9]+),(?<op2>[0-9]+)\))|(?<dt>don't\(\))|(?<d>do\(\))").unwrap()
});

pub fn part_one(input: &str) -> Option<u32> {
    let mut result = 0;
    for c in RE_1.captures_iter(input) {
        result += c[1].parse::<u32>().unwrap() * c[2].parse::<u32>().unwrap();
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result = 0;
    let mut valid = false;
    for m in RE_2.captures_iter(input) {
        if m.name("d").is_some() {
            valid = true;
        } else if m.name("dt").is_some() {
            valid = false;
        }
        if valid {
            if let (Some(op1), Some(op2)) = (m.name("op1"), m.name("op2")) {
                result +=
                    op1.as_str().parse::<u32>().unwrap() * op2.as_str().parse::<u32>().unwrap();
            }
        }
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
