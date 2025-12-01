pub fn solve(input: &str) -> u64 {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    println!("Grid {:?}", grid);

    let mut scan_row = 0;

    const DIRECTIONS: &[(isize, isize)] = &[
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut total = 0;

    while scan_row < grid.len() {
        let mut c = 0;
        while c < grid[scan_row].len() {
            if grid[scan_row][c].is_numeric() {
                let num = grid[scan_row]
                    .iter()
                    .skip(c)
                    .take_while(|&&ch| ch.is_numeric())
                    .collect::<String>()
                    .parse::<u64>()
                    .unwrap();
                let num_len = num.ilog10() as usize + 1;

                'neighbor_search: for nc in c..c + num_len {
                    for &(dr, dc) in DIRECTIONS {
                        let nnc = nc as isize + dc;
                        let nnr = scan_row as isize + dr;
                        if nnr >= 0
                            && nnr < grid.len() as isize
                            && nnc >= 0
                            && nnc < grid[nnr as usize].len() as isize
                        {
                            let neighbor = grid[nnr as usize][nnc as usize];
                            if !neighbor.is_numeric() && neighbor != '.' {
                                println!(
                                    "[row {}] Number {} is adjacent to '{}', adding to total",
                                    scan_row, num, neighbor
                                );
                                total += num;
                                break 'neighbor_search;
                            }
                        }
                    }
                }

                c += num_len;
            }
            c += 1;
        }
        scan_row += 1;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("./../test.txt");
        assert_eq!(solve(input), 4361);
    }
}
