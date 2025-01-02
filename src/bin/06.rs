use std::collections::HashSet;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = input_to_map(input);
    let height = map.len();
    let width = map[0].len();
    let mut pos = find_start(&map);
    let mut history: HashSet<(i32, i32)> = HashSet::new();
    history.insert((pos.0 as i32, pos.1 as i32));
    let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut dir_idx = 0;
    loop {
        let direction = dirs[dir_idx];
        let next = next(pos, direction);
        if !bound_check(next.0, next.1, width, height) {
            return Some(history.len() as u32);
        }
        if map[next.0 as usize][next.1 as usize] == '.' {
            history.insert(next);
            map[pos.0][pos.1] = '.';
            pos = (next.0 as usize, next.1 as usize);
        } else {
            dir_idx = (dir_idx + 1) % 4;
        }
    }
}

fn bound_check(x: i32, y: i32, width: usize, height: usize) -> bool {
    x >= 0 && x < height as i32 && y >= 0 && y < width as i32
}

fn next(actual: (usize, usize), direction: (i32, i32)) -> (i32, i32) {
    (
        (actual.0 as i32 + direction.0),
        (actual.1 as i32 + direction.1),
    )
}

fn find_start(map: &[Vec<char>]) -> (usize, usize) {
    for (line_number, line) in map.iter().enumerate() {
        for (column, ch) in line.iter().enumerate() {
            if *ch == '^' {
                return (line_number, column);
            }
        }
    }
    (0, 0)
}

fn out_of_grid2(start: &(i32, i32, usize), m: &[Vec<char>]) -> Option<i32> {
    let width = m[0].len();
    let height = m.len();
    // directions in an anticlockwise way: up, right, down, left
    let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let (mut cur_x, mut cur_y, mut cur_idx) = start;
    // optimisation 3: using a 2D array rather than a hashset
    let mut hits = vec![vec![[false; 4]; m[0].len()]; m.len()];
    loop {
        if hits[cur_x as usize][cur_y as usize][cur_idx] {
            return None;
        }
        hits[cur_x as usize][cur_y as usize][cur_idx] = true;

        let cur_step = dirs[cur_idx];
        let (new_x, new_y) = (cur_x + cur_step.0, cur_y + cur_step.1);
        if !bound_check(new_x, new_y, width, height) {
            return Some(
                (hits
                    .iter()
                    .enumerate()
                    .flat_map(|(i, row)| {
                        row.iter()
                            .enumerate()
                            .filter(|&(_, col)| col.iter().any(|&b| b))
                            .map(move |(j, _)| (i, j))
                    })
                    .collect::<Vec<(usize, usize)>>()
                    .len()) as i32,
            );
        }
        if m[new_x as usize][new_y as usize] == '#' {
            cur_idx = (cur_idx + 1) % 4;
        } else {
            (cur_x, cur_y) = (new_x, new_y);
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut m = input_to_map(input);
    let start = find_start(&m);
    let start = (start.0 as i32, start.1 as i32);
    m[start.0 as usize][start.1 as usize] = '.';

    let width = m[0].len();
    let height = m.len();

    let dirs: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut cur_idx = 0;
    let (mut cur_x, mut cur_y) = (start.0, start.1);

    // optimisation 3: use 2D array instead of a hashset
    let mut visited = vec![vec![false; width]; height];
    let mut res = 0;
    loop {
        visited[cur_x as usize][cur_y as usize] = true;
        let cur_step = dirs[cur_idx];
        let (new_x, new_y) = (cur_x + cur_step.0, cur_y + cur_step.1);
        if !bound_check(new_x, new_y, width, height) {
            break;
        }
        if m[new_x as usize][new_y as usize] == '#' {
            cur_idx = (cur_idx + 1) % 4;
        } else {
            if (new_x, new_y) != start && !visited[new_x as usize][new_y as usize] {
                m[new_x as usize][new_y as usize] = '#';
                if out_of_grid2(&(cur_x, cur_y, cur_idx), &m).is_none() {
                    res += 1;
                }
                m[new_x as usize][new_y as usize] = '.';
            }
            (cur_x, cur_y) = (new_x, new_y);
        }
    }
    Some(res)
}

fn input_to_map(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
