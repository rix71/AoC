use cached::SizedCache;
use cached::proc_macro::cached;

#[cached(
    ty = "SizedCache<String, u64>",
    create = "{ SizedCache::with_size(150) }",
    convert = r#"{ format!("{}{}", r, c) }"#
)]
fn step(r: usize, c: usize, manifold: &Vec<Vec<char>>, max_levels: usize) -> u64 {
    if r + 1 == max_levels {
        return 1;
    }
    if manifold[r + 1][c] == '^' {
        return step(r + 1, c - 1, manifold, max_levels) + step(r + 1, c + 1, manifold, max_levels);
    } else {
        return step(r + 1, c, manifold, max_levels);
    }
}

pub fn solve(input: &str) -> u64 {
    let manifold = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let max_levels = manifold.len();
    let start_pos = manifold[0].iter().position(|c| *c == 'S').unwrap();
    step(0, start_pos, &manifold, max_levels)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = include_str!("../test.txt");
        assert_eq!(solve(input), 40);
    }
}
