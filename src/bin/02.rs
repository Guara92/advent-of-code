advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    evaluate_reports(input, 0)
}

pub fn part_two(input: &str) -> Option<u32> {
    evaluate_reports(input, 1)
}

fn evaluate_reports(input: &str, tollerance: u8) -> Option<u32> {
    let reports = input.lines();
    let mut safe = 0;
    for line in reports {
        let levels = line.split_whitespace();
        if is_line_ok(levels.clone(), tollerance) || is_line_ok(levels.rev(), tollerance) {
            safe += 1;
        }
    }
    Some(safe)
}

fn is_line_ok<'a>(levels: impl Iterator<Item = &'a str>, tollerance: u8) -> bool {
    let mut tollerance = tollerance;
    let mut previous = 0;
    let mut level_ok = true;
    let mut report_ok = true;
    let mut incremental = None;
    for (pos, level) in levels.enumerate() {
        if pos == 0 {
            previous = level.parse::<u32>().unwrap();
            continue;
        }
        let actual = level.parse::<u32>().unwrap();
        if incremental.is_none() {
            match previous.cmp(&actual) {
                std::cmp::Ordering::Less => {
                    incremental = Some(true);
                }
                std::cmp::Ordering::Equal => {
                    if tollerance == 0 {
                        report_ok = false;
                        break;
                    } else {
                        tollerance -= 1;
                        continue;
                    }
                }
                std::cmp::Ordering::Greater => {
                    incremental = Some(false);
                }
            }
        }
        match (
            is_tuple_safe(previous, actual, incremental.unwrap()),
            tollerance,
        ) {
            (true, _) => {
                previous = actual;
            }
            (false, 0) => {
                level_ok = false;
                break;
            }
            (false, _) => {
                tollerance -= 1;
            }
        }
    }
    level_ok && report_ok
}

fn is_tuple_safe(previous: u32, actual: u32, incremental: bool) -> bool {
    if incremental {
        previous < actual && actual - previous <= 3
    } else {
        previous > actual && previous - actual <= 3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }
}
