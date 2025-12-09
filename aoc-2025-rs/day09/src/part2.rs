use glam::U64Vec2;
use itertools::Itertools;

pub fn solve(input: &str) -> u64 {
    let positions = input
        .lines()
        .map(|line| {
            let (x, y) = line
                .split_once(',')
                .map(|(x, y)| (x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap()))
                .unwrap();
            U64Vec2::new(x, y)
        })
        .collect::<Vec<_>>();
    
    let lines = positions
        .iter()
        .circular_tuple_windows()
        .collect::<Vec<(&U64Vec2, &U64Vec2)>>();

    let max_area = positions
        .iter()
        .combinations(2)
        .map(|p| {
            let dx = (p[0].x.max(p[1].x) - p[0].x.min(p[1].x)) + 1;
            let dy = (p[0].y.max(p[1].y) - p[0].y.min(p[1].y)) + 1;
            let area = dx * dy;
            (p[0], p[1], area)
        })
        .sorted_by_key(|p| p.2)
        .rev()
        .find(|(p1, p2, _area)| {
            for (ls, le) in lines.iter() {
                let line_is_left = p1.x.min(p2.x) >= ls.x.max(le.x);
                let line_is_right = p1.x.max(p2.x) <= ls.x.min(le.x);
                let line_is_above = p1.y.min(p2.y) >= ls.y.max(le.y);
                let line_is_below = p1.y.max(p2.y) <= ls.y.min(le.y);
                if !(line_is_left || line_is_right || line_is_above || line_is_below) {
                    return false;
                }
            }
            true
        })
        .unwrap()
        .2;

    max_area
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = include_str!("../test.txt");
        assert_eq!(solve(input), 24);
    }
}
