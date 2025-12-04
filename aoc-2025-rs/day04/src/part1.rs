const NEIGHBORS: [(isize, isize); 8] = [
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
];

pub fn solve(input: &str) -> u64 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut total = 0;

    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == '@' {
                let mut num_neighbours = 0;
                for (dx, dy) in NEIGHBORS.iter() {
                    let ni = i as isize + dx;
                    let nj = j as isize + dy;
                    if ni < 0 || nj < 0 || ni >= rows as isize || nj >= cols as isize {
                        continue;
                    }
                    if grid[ni as usize][nj as usize] == '@' {
                        num_neighbours += 1
                    }
                }
                if num_neighbours < 4 {
                    total += 1;
                }
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../test.txt");
        assert_eq!(solve(input), 13);
    }
}
