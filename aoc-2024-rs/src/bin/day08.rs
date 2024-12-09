use std::collections::{HashMap, HashSet};

fn part1(grid: &Vec<Vec<char>>) {
    let mut nodes: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let rows = grid.len();
    let cols = grid[0].len();
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] != '.' {
                nodes
                    .entry(grid[r][c])
                    .or_insert(vec![])
                    .push((r as i32, c as i32));
            }
        }
    }
    let mut antinodes = HashSet::new();
    nodes.iter().for_each(|(_, coords)| {
        for i in 0..coords.len() {
            let (x1, y1) = coords[i];
            for j in i + 1..coords.len() {
                let (x2, y2) = coords[j];
                let dx = x2 - x1;
                let dy = y2 - y1;
                let an1 = (x1 + 2 * dx, y1 + 2 * dy);
                let an2 = (x2 - 2 * dx, y2 - 2 * dy);
                if an1.0 >= 0 && an1.0 < rows as i32 && an1.1 >= 0 && an1.1 < cols as i32 {
                    antinodes.insert(an1);
                }
                if an2.0 >= 0 && an2.0 < rows as i32 && an2.1 >= 0 && an2.1 < cols as i32 {
                    antinodes.insert(an2);
                }
            }
        }
    });
    println!("{:?}", antinodes.len());
}

fn part2(grid: &Vec<Vec<char>>) {
    let mut nodes: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let rows = grid.len();
    let cols = grid[0].len();
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] != '.' {
                nodes
                    .entry(grid[r][c])
                    .or_insert(vec![])
                    .push((r as i32, c as i32));
            }
        }
    }
    let mut antinodes = HashSet::new();
    nodes.iter().for_each(|(_, coords)| {
        for i in 0..coords.len() {
            let (x1, y1) = coords[i];
            for j in i + 1..coords.len() {
                let (x2, y2) = coords[j];
                let dx = x2 - x1;
                let dy = y2 - y1;
                for ix in 0..cols {
                    let an1 = (x1 + ix as i32 * dx, y1 + ix as i32 * dy);
                    let an2 = (x2 - ix as i32 * dx, y2 - ix as i32 * dy);
                    if an1.0 >= 0 && an1.0 < rows as i32 && an1.1 >= 0 && an1.1 < cols as i32 {
                        antinodes.insert(an1);
                    }
                    if an2.0 >= 0 && an2.0 < rows as i32 && an2.1 >= 0 && an2.1 < cols as i32 {
                        antinodes.insert(an2);
                    }
                }
            }
        }
    });
    println!("{:?}", antinodes.len());
}

fn main() {
    let input = include_str!("../../input/day08/in.txt");
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    println!("{:?}", grid);
    part1(&grid);
    part2(&grid);
}
