use std::vec;

fn directions() -> Vec<(i32, i32)> {
    let mut dirs = vec![];
    for dx in -1..2 {
        for dy in -1..2 {
            if dx != 0 || dy != 0 {
                dirs.push((dx, dy));
            }
        }
    }
    dirs
}

fn part1(grid: Vec<&str>) {
    let dirs = directions();

    let has_xmas = |i, j, d: &(i32, i32)| {
        let (dx, dy) = d;
        const TARGET: &str = "XMAS";
        for k in 0..TARGET.len() {
            let ni = i as i32 + k as i32 * dx;
            let nj = j as i32 + k as i32 * dy;
            if ni < 0 || ni >= grid.len() as i32 || nj < 0 || nj >= grid[0].len() as i32 {
                return false;
            }
            if grid[ni as usize].chars().nth(nj as usize).unwrap() != TARGET.chars().nth(k).unwrap()
            {
                return false;
            }
        }
        true
    };

    let mut count = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            for d in dirs.iter() {
                if has_xmas(i, j, d) {
                    count += 1;
                }
            }
        }
    }
    println!("{:?}", count);
}

fn part2(grid: Vec<&str>) {
    let has_xmas = |i, j| {
        if i == 0 || i > grid.len() - 2 || j == 0 || j > grid[0].len() - 2 {
            return false;
        }
        let middle = grid[i as usize].chars().nth(j).unwrap();
        if middle != 'A' {
            return false;
        }

        let mut d1 = grid[(i - 1) as usize]
            .chars()
            .nth(j - 1)
            .unwrap()
            .to_string();
        d1.push(grid[(i + 1) as usize].chars().nth(j + 1).unwrap());
        let mut d2 = grid[(i - 1) as usize]
            .chars()
            .nth(j + 1)
            .unwrap()
            .to_string();
        d2.push(grid[(i + 1) as usize].chars().nth(j - 1).unwrap());

        (d1 == "MS" || d1 == "SM") && (d2 == "MS" || d2 == "SM")
    };

    let mut count = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if has_xmas(i, j) {
                count += 1;
            }
        }
    }
    println!("{:?}", count);
}

fn main() {
    let input = include_str!("../../input/day04/in.txt");
    let grid = input.lines().collect::<Vec<_>>();
    part1(grid.clone());
    part2(grid.clone());
}
