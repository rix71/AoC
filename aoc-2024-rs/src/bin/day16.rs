use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn part1(maze: &Vec<Vec<char>>) -> Option<i32> {
    let rows = maze.len();
    let cols = maze[0].len();

    let mut start = (0, 0);
    for r in 0..rows {
        for c in 0..cols {
            if maze[r][c] == 'S' {
                start = (r, c);
                break;
            }
        }
    }

    let mut seen = HashSet::new();
    let mut tiles = BinaryHeap::new();
    tiles.push(Reverse((0, start, 0)));

    while let Some(Reverse((pts, (r, c), cdir))) = tiles.pop() {
        seen.insert(((r, c), cdir));
        for (idir, (dr, dc)) in DIRECTIONS.iter().enumerate() {
            if (*dr, *dc) == DIRECTIONS[(cdir + 2) % 4] {
                continue;
            }
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr < 0 || nr >= rows as i32 || nc < 0 || nc >= cols as i32 {
                continue;
            }
            if maze[nr as usize][nc as usize] == '#'
                || seen.contains(&((nr as usize, nc as usize), idir))
            {
                continue;
            }
            if maze[nr as usize][nc as usize] == 'E' {
                return Some(pts + 1);
            }
            if idir == cdir {
                tiles.push(Reverse((pts + 1, (nr as usize, nc as usize), idir)));
            } else {
                tiles.push(Reverse((pts + 1000, (r as usize, c as usize), idir)));
            }
        }
    }

    None
}

fn part2(maze: &Vec<Vec<char>>) -> Option<i32> {
    let rows = maze.len();
    let cols = maze[0].len();

    let mut start = (0, 0);
    for r in 0..rows {
        for c in 0..cols {
            if maze[r][c] == 'S' {
                start = (r, c);
                break;
            }
        }
    }

    let mut seen = HashSet::new();
    let mut on_path = HashMap::new();
    let mut tiles = BinaryHeap::new();
    tiles.push(Reverse((0, start, 0, vec![start])));

    while let Some(Reverse((pts, (r, c), cdir, history))) = tiles.pop() {
        seen.insert(((r, c), cdir));
        for (idir, (dr, dc)) in DIRECTIONS.iter().enumerate() {
            if (*dr, *dc) == DIRECTIONS[(cdir + 2) % 4] {
                continue;
            }
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr < 0 || nr >= rows as i32 || nc < 0 || nc >= cols as i32 {
                continue;
            }
            if maze[nr as usize][nc as usize] == '#'
                || seen.contains(&((nr as usize, nc as usize), idir))
            {
                continue;
            }
            if maze[nr as usize][nc as usize] == 'E' {
                let mut path_history = history.clone();
                path_history.push((nr as usize, nc as usize));
                on_path
                    .entry(pts + 1)
                    .and_modify(|p: &mut Vec<(usize, usize)>| p.extend(path_history.iter()))
                    .or_insert(path_history);
                continue;
            }
            if idir == cdir {
                let mut path_history = history.clone();
                path_history.push((nr as usize, nc as usize));
                tiles.push(Reverse((
                    pts + 1,
                    (nr as usize, nc as usize),
                    idir,
                    path_history,
                )));
            } else {
                let mut path_history = history.clone();
                path_history.push((r as usize, c as usize));
                tiles.push(Reverse((
                    pts + 1000,
                    (r as usize, c as usize),
                    idir,
                    path_history,
                )));
            }
        }
    }

    if on_path.is_empty() {
        return None;
    } else {
        let min_path = on_path.keys().min().unwrap();
        let result = on_path
            .get(min_path)
            .unwrap()
            .iter()
            .fold(HashSet::new(), |mut path_locs, loc| {
                path_locs.insert(loc);
                path_locs
            })
            .len();
        return Some(result as i32);
    }
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn main() {
    let input = include_str!("../../input/day16/in.txt");
    let maze = parse(input);
    println!("Part 1: {:?}", part1(&maze));
    println!("Part 2: {:?}", part2(&maze));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1_part1() {
        let input = include_str!("../../input/day16/test1.txt");
        let maze = parse(input);
        assert_eq!(part1(&maze), Some(7036));
    }

    #[test]
    fn test_case2_part1() {
        let input = include_str!("../../input/day16/test2.txt");
        let maze = parse(input);
        assert_eq!(part1(&maze), Some(11048));
    }

    #[test]
    fn test_case1_part2() {
        let input = include_str!("../../input/day16/test1.txt");
        let maze = parse(input);
        assert_eq!(part2(&maze), Some(45));
    }

    #[test]
    fn test_case2_part2() {
        let input = include_str!("../../input/day16/test2.txt");
        let maze = parse(input);
        assert_eq!(part2(&maze), Some(64));
    }
}
