use std::{collections::HashSet, vec};

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn part1(topo: &Vec<Vec<u32>>) {
    let rows = topo.len();
    let cols = topo[0].len();

    let trace_path = |r: usize, c: usize| {
        let mut trails = vec![];
        let mut destinations = HashSet::new();
        let mut visited = vec![vec![false; cols]; rows];

        trails.push((0, (r as i32, c as i32)));

        while !trails.is_empty() {
            let (trail_val, (tr, tc)) = trails.pop().unwrap();
            visited[tr as usize][tc as usize] = true;
            for (dr, dc) in DIRECTIONS.iter() {
                let nr = tr + dr;
                let nc = tc + dc;
                if nr < 0 || nr >= rows as i32 || nc < 0 || nc >= cols as i32 {
                    continue;
                }
                if visited[nr as usize][nc as usize] {
                    continue;
                }
                if trail_val == 8 && topo[nr as usize][nc as usize] == 9 {
                    destinations.insert((nr, nc));
                    continue;
                }
                if topo[nr as usize][nc as usize] == trail_val + 1 {
                    trails.push((trail_val + 1, (nr, nc)));
                }
            }
        }
        destinations.len()
    };

    let mut score = 0;
    for r in 0..rows {
        for c in 0..cols {
            if topo[r][c] == 0 {
                score += trace_path(r, c);
            }
        }
    }
    println!("{:?}", score);
}

fn part2(topo: &Vec<Vec<u32>>) {
    let rows = topo.len();
    let cols = topo[0].len();

    let trace_path = |r: usize, c: usize| {
        let mut trails = vec![];
        let mut trail_score = 0;

        trails.push((0, (r as i32, c as i32)));

        while !trails.is_empty() {
            let (trail_val, (tr, tc)) = trails.pop().unwrap();
            for (dr, dc) in DIRECTIONS.iter() {
                let nr = tr + dr;
                let nc = tc + dc;
                if nr < 0 || nr >= rows as i32 || nc < 0 || nc >= cols as i32 {
                    continue;
                }
                if trail_val == 8 && topo[nr as usize][nc as usize] == 9 {
                    trail_score += 1;
                    continue;
                }
                if topo[nr as usize][nc as usize] == trail_val + 1 {
                    trails.push((trail_val + 1, (nr, nc)));
                }
            }
        }
        trail_score
    };

    let mut score = 0;
    for r in 0..rows {
        for c in 0..cols {
            if topo[r][c] == 0 {
                score += trace_path(r, c);
            }
        }
    }
    println!("{:?}", score);
}

fn main() {
    let input = include_str!("../../input/day10/in.txt");
    let topo: Vec<_> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    // println!("{:?}", topo);
    part1(&topo);
    part2(&topo);
}
