use glam::U64Vec2;
use itertools::Itertools;

pub fn solve(input: &str) -> u64 {
    let max_area = input
        .lines()
        .map(|line| {
            U64Vec2::from(
                line.split_once(',')
                    .map(|(x, y)| (x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap()))
                    .unwrap(),
            )
        })
        .combinations(2)
        .map(|p| {
            let dx = (p[0].x.max(p[1].x) - p[0].x.min(p[1].x)) + 1;
            let dy = (p[0].y.max(p[1].y) - p[0].y.min(p[1].y)) + 1;
            let area = dx * dy;
            println!(
                "Points: {:?}, {:?} => Area: {} x {} = {}",
                p[0], p[1], dx, dy, area
            );
            assert!(area > 0);
            area
        })
        .max()
        .unwrap();
    // .collect::<Vec<_>>();

    max_area
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../test.txt");
        assert_eq!(solve(input), 50);
    }
}
