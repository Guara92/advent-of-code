advent_of_code::solution!(4);

type Point = (i64, i64);
const PATTERN_1_COORDINATES: [[Point; 3]; 8] = [
    [(1, 0), (2, 0), (3, 0)],
    [(1, -1), (2, -2), (3, -3)],
    [(0, -1), (0, -2), (0, -3)],
    [(-1, -1), (-2, -2), (-3, -3)],
    [(-1, 1), (-2, 2), (-3, 3)],
    [(-1, 0), (-2, 0), (-3, 0)],
    [(0, 1), (0, 2), (0, 3)],
    [(1, 1), (2, 2), (3, 3)],
];

const PATTERNS_2_COORDINATES: [[(Point, Point); 2]; 4] =
    //  a tuple of 2 point define half of a pattern, so we need 2 tuple to define a full pattern
    // [[(xM, yM), (xS, yS)], [(xM, yM), (xS, yS)]]
    [
        [((-1, -1), (1, 1)), ((-1, 1), (1, -1))],
        [((-1, -1), (1, 1)), ((1, -1), (-1, 1))],
        [((1, 1), (-1, -1)), ((-1, 1), (1, -1))],
        [((1, -1), (-1, 1)), ((1, 1), (-1, -1))],
    ];

pub fn part_one(input: &str) -> Option<u32> {
    let mut occurrences = 0;
    let matrix: Vec<Vec<char>> = input_to_matrix(input);
    for (y, line) in matrix.iter().enumerate() {
        for (x, chars) in line.iter().enumerate() {
            occurrences += match chars {
                'X' => find_pattern(&matrix, x, y, false),
                'S' => find_pattern(&matrix, x, y, true),
                _ => 0,
            };
        }
    }
    Some(occurrences)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut occurrences = 0;
    let matrix: Vec<Vec<char>> = input_to_matrix(input);
    for (y, line) in matrix.iter().enumerate() {
        for (x, _) in line.iter().enumerate().filter(|&(_, &c)| c == 'A') {
            if find_pattern_2(&matrix, x, y) {
                occurrences += 1;
            }
        }
    }
    Some(occurrences)
}

fn input_to_matrix(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

fn find_pattern(input: &[Vec<char>], col: usize, row: usize, rev: bool) -> u32 {
    let search_str = if rev {
        ['S', 'A', 'M']
    } else {
        ['M', 'A', 'S']
    };
    let mut occurrences = 0;
    for pattern in PATTERN_1_COORDINATES.iter() {
        let mut found = true;
        for (i, (x, y)) in pattern.iter().enumerate() {
            if row as i64 + y < 0
                || row as i64 + y >= input.len() as i64
                || col as i64 + x < 0
                || col as i64 + x >= input[row].len() as i64
            {
                found = false;
                break;
            }
            if input[(row as i64 + y) as usize][(col as i64 + x) as usize] != search_str[i] {
                found = false;
                break;
            }
        }
        if found {
            occurrences += 1;
        }
    }
    occurrences
}

fn find_pattern_2(input: &[Vec<char>], col: usize, row: usize) -> bool {
    if row == 0 || col == 0 || row == input.len() - 1 || col == input[row].len() - 1 {
        return false;
    }
    for pattern in PATTERNS_2_COORDINATES.iter() {
        let mut found = 0;
        for part in pattern.iter() {
            let (m_x, m_y) = part.0;
            let (s_x, s_y) = part.1;
            if row as i64 + m_y < 0
                || row as i64 + s_y < 0
                || row as i64 + m_y >= input.len() as i64
                || row as i64 + s_y >= input.len() as i64
                || col as i64 + m_x < 0
                || col as i64 + s_x < 0
                || col as i64 + m_x >= input[row].len() as i64
                || col as i64 + s_x >= input[row].len() as i64
            {
                break;
            }
            if input[(row as i64 + m_y) as usize][(col as i64 + m_x) as usize] != 'M'
                || input[(row as i64 + s_y) as usize][(col as i64 + s_x) as usize] != 'S'
            {
                break;
            } else {
                found += 1;
            }
        }
        if found == 2 {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
