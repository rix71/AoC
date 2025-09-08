use std::collections::HashMap;

pub fn solve(input: &str) -> u32 {
    let cube_idx = HashMap::from([("red", 0), ("green", 1), ("blue", 2)]);
    let mut total = 0;

    input.lines().for_each(|line| {
        println!("--------------------------------");
        let (_game, info) = line.split_once(": ").unwrap();
        let mut min_cubes = vec![1, 1, 1];
        info.split("; ").for_each(|sub| {
            sub.split(", ").for_each(|cube| {
                let (num, color) = cube.split_once(" ").unwrap();
                let num: u32 = num.parse().unwrap();
                let idx = cube_idx.get(color).unwrap();
                min_cubes[*idx] = min_cubes[*idx].max(num);
            });
        });
        let power = min_cubes.iter().product::<u32>();
        println!("{:?} -> {}", min_cubes, power);
        total += power;
    });
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = include_str!("../test.txt");
        let result = solve(input);
        assert_eq!(result, 2286);
    }
}
