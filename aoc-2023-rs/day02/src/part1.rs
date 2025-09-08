use std::collections::HashMap;

pub fn solve(input: &str) -> u32 {
    let max_cubes = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let mut total = 0;

    input.lines().for_each(|line| {
        let mut ok = true;
        let (game, info) = line.split_once(": ").unwrap();
        info.split("; ").for_each(|sub| {
            sub.split(", ").for_each(|cube| {
                let (num, color) = cube.split_once(" ").unwrap();
                let num: u32 = num.parse().unwrap();
                let max = max_cubes.get(color).unwrap();
                if num > *max {
                    ok = false;
                }
            });
        });
        if ok {
            let (_, game_id) = game.split_once(" ").unwrap();
            let game_id = game_id.parse::<u32>().unwrap();
            total += game_id;
        }
    });
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../test.txt");
        let result = solve(input);
        assert_eq!(result, 8);
    }
}
