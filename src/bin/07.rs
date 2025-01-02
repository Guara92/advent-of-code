advent_of_code::solution!(7);

struct Equation {
    result: u64,
    values: Vec<u64>,
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut result = 0;
    for line in input.lines() {
        let mut eq = line.split(":");
        let res = eq.next().unwrap().parse::<u64>().unwrap();
        let values: Vec<u64> = eq
            .next()
            .unwrap()
            .split_whitespace()
            .map(|val| val.parse::<u64>().unwrap())
            .collect();
        let eq = Equation::new(res, values);
        if eq.is_valid(eq.values[0], &eq.values[1..]) {
            result += eq.result;
        }
    }
    Some(result)
}

impl Equation {
    pub fn new(result: u64, values: Vec<u64>) -> Self {
        Self { result, values }
    }

    fn is_valid(&self, a: u64, values: &[u64]) -> bool {
        if values.is_empty() {
            return self.result == a;
        }
        let (h, t) = (values[0], &values[1..]);
        self.is_valid(a * h, t) || self.is_valid(a + h, t)
    }

    fn is_valid2(&self, a: u64, nums: &[u64]) -> bool {
        if nums.len() == 0 {
            return self.result == a;
        }
        let (h, t) = (nums[0], &nums[1..]);
        return self.is_valid2(a * h, t)
            || self.is_valid2(a + h, t)
            || self.is_valid2(
                [a.to_string(), h.to_string()]
                    .concat()
                    .parse::<u64>()
                    .unwrap(),
                &t,
            );
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut result = 0;
    for line in input.lines() {
        let mut eq = line.split(":");
        let res = eq.next().unwrap().parse::<u64>().unwrap();
        let values: Vec<u64> = eq
            .next()
            .unwrap()
            .split_whitespace()
            .map(|val| val.parse::<u64>().unwrap())
            .collect();
        let eq = Equation::new(res, values);
        if eq.is_valid2(eq.values[0], &eq.values[1..]) {
            result += eq.result;
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
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
