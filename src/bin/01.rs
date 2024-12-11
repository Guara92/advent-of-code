use std::collections::{BinaryHeap, HashMap};

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.lines();
    let mut right = BinaryHeap::with_capacity(lines.size_hint().0);
    let mut left = BinaryHeap::with_capacity(lines.size_hint().0);
    for line in lines {
        let mut el = line.split_whitespace();
        left.push(el.next().unwrap().parse::<u32>().unwrap());
        right.push(el.last().unwrap().parse::<u32>().unwrap());
    }
    let mut result = 0;
    while let Some(l) = left.pop() {
        let r = right.pop().unwrap();
        result += (r as i64 - l as i64).unsigned_abs();
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut left: HashMap<u32, u32> = HashMap::with_capacity(lines.size_hint().0);
    let mut right: HashMap<u32, u32> = HashMap::with_capacity(lines.size_hint().0);
    for line in lines {
        let mut el = line.split_whitespace();
        left.entry(el.next().unwrap().parse::<u32>().unwrap())
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
        right
            .entry(el.last().unwrap().parse::<u32>().unwrap())
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
    let mut result = 0;
    for (value, occurrencies) in left.iter() {
        result += value * occurrencies * right.get(value).unwrap_or(&0);
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
