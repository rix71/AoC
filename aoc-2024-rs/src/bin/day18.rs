use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn part1(bytes: Vec<(i32, i32)>, nbytes: usize, rows: usize, cols: usize) -> Option<i32> {
    let mut grid = vec![vec!['.'; cols]; rows];

    let start = (0, 0);
    let end = (cols - 1, rows - 1);

    bytes.iter().take(nbytes).for_each(|(x, y)| {
        grid[*y as usize][*x as usize] = '#';
    });

    let mut seen = HashSet::new();
    let mut look = BinaryHeap::new();
    look.push(Reverse((0, start, 0)));

    while let Some(Reverse((steps, (r, c), cdir))) = look.pop() {
        for (idir, (dr, dc)) in DIRECTIONS.iter().enumerate() {
            if (*dr, *dc) == DIRECTIONS[(cdir + 2) % 4] {
                continue;
            }
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr < 0 || nr >= rows as i32 || nc < 0 || nc >= cols as i32 {
                continue;
            }
            if grid[nr as usize][nc as usize] == '#' || seen.contains(&(nr as usize, nc as usize)) {
                continue;
            }
            if (nr as usize, nc as usize) == end {
                return Some(steps + 1);
            }

            seen.insert((nr as usize, nc as usize));
            look.push(Reverse((steps + 1, (nr as usize, nc as usize), idir)));
        }
    }

    None
}

fn part2(bytes: Vec<(i32, i32)>, rows: usize, cols: usize) -> Option<(i32, i32)> {
    let end = (cols - 1, rows - 1);

    let mut low = 0;
    let mut high = bytes.len() - 1;
    let get_mid = |low: usize, high: usize| (low + high) / 2;
    let mut mid = get_mid(low, high);

    while low < high {
        let exit_possible = |nbytes: usize| {
            let mut grid = vec![vec!['.'; cols]; rows];
            bytes.iter().take(nbytes).for_each(|(x, y)| {
                grid[*y as usize][*x as usize] = '#';
            });
            let mut seen = HashSet::new();
            let mut look = BinaryHeap::new();
            look.push(Reverse((0, (0, 0), 0)));

            while let Some(Reverse((steps, (r, c), cdir))) = look.pop() {
                for (idir, (dr, dc)) in DIRECTIONS.iter().enumerate() {
                    if (*dr, *dc) == DIRECTIONS[(cdir + 2) % 4] {
                        continue;
                    }
                    let nr = r as i32 + dr;
                    let nc = c as i32 + dc;
                    if nr < 0 || nr >= rows as i32 || nc < 0 || nc >= cols as i32 {
                        continue;
                    }
                    if grid[nr as usize][nc as usize] == '#'
                        || seen.contains(&(nr as usize, nc as usize))
                    {
                        continue;
                    }
                    if (nr as usize, nc as usize) == end {
                        return true;
                    }

                    seen.insert((nr as usize, nc as usize));
                    look.push(Reverse((steps + 1, (nr as usize, nc as usize), idir)));
                }
            }
            false
        };
        if exit_possible(mid+1) {
            low = mid + 1;
        } else {
            if low + 1 == mid {
                return Some(bytes[mid]);
            } else {
                high = mid;
            }
        }
        mid = get_mid(low, high);
    }
    None
}

fn main() {
    let input = include_str!("../../input/day18/in.txt");
    let bytes = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            let x = x.parse::<i32>().unwrap();
            let y = y.parse::<i32>().unwrap();
            (x, y)
        })
        .collect::<Vec<_>>();

    println!("Part 1: {:?}", part1(bytes.clone(), 1024, 71, 71).unwrap());
    println!("Part 2: {:?}", part2(bytes, 71, 71).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/day18/test.txt");
        let bytes = input
            .lines()
            .map(|line| {
                let (x, y) = line.split_once(",").unwrap();
                let x = x.parse::<i32>().unwrap();
                let y = y.parse::<i32>().unwrap();
                (x, y)
            })
            .collect::<Vec<_>>();

        assert_eq!(part1(bytes, 12, 7, 7), Some(22));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/day18/test.txt");
        let bytes = input
            .lines()
            .map(|line| {
                let (x, y) = line.split_once(",").unwrap();
                let x = x.parse::<i32>().unwrap();
                let y = y.parse::<i32>().unwrap();
                (x, y)
            })
            .collect::<Vec<_>>();

        assert_eq!(part2(bytes, 7, 7), Some((6, 1)));
    }
}
