pub fn solve(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let nums = line
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<_>>();
            nums[0] * 10 + nums.last().unwrap()
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../test.txt");
        let result = solve(input);
        assert_eq!(result, 142);
    }
}
