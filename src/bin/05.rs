use std::collections::HashMap;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, updates) = parse_input(input);

    let mut result = 0;
    for update in updates {
        if is_update_valid(&rules, &update) {
            result += update.get(update.len() / 2).unwrap();
        }
    }
    Some(result)
}

fn parse_input(input: &str) -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut updates = vec![];
    let mut lines = input.lines();
    let mut line = lines.next();
    while line.is_some_and(|line| !line.is_empty()) {
        let mut el = line.unwrap().split('|');
        let a = el.next().unwrap().parse::<u32>().unwrap();
        let b = el.next().unwrap().parse::<u32>().unwrap();
        line = lines.next();
        rules
            .entry(b)
            .and_modify(|previous| previous.push(a))
            .or_insert(vec![a]);
    }
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let mut update = vec![];
        let elements = line.split(',');
        for el in elements {
            update.push(el.parse::<u32>().unwrap());
        }
        updates.push(update);
    }
    (rules, updates)
}

fn is_update_valid(rules: &HashMap<u32, Vec<u32>>, update: &[u32]) -> bool {
    let mut pages_to_update = update.iter();
    let mut before = pages_to_update.next().unwrap();

    for page_to_update in pages_to_update {
        if let Some(legit_updates) = rules.get(page_to_update) {
            if !legit_updates.contains(before) {
                return false;
            }
        } else if let Some(invalid_updates) = rules.get(before) {
            if invalid_updates.contains(page_to_update) {
                return false;
            }
        }
        before = page_to_update;
    }

    true
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, updates) = parse_input(input);

    let mut result = 0;
    for mut update in updates {
        if !is_update_valid(&rules, &update) {
            result += get_value_from_invalid(&rules, &mut update);
        }
    }
    Some(result)
}

fn get_value_from_invalid(rules: &HashMap<u32, Vec<u32>>, update: &mut [u32]) -> u32 {
    update.sort_by(|a, b| {
        if let Some(rules) = rules.get(b) {
            if rules.contains(a) {
                return std::cmp::Ordering::Less;
            }
        }
        if let Some(rules) = rules.get(a) {
            if rules.contains(b) {
                return std::cmp::Ordering::Greater;
            }
        }
        a.cmp(b)
    });
    *update.get(update.len() / 2).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
