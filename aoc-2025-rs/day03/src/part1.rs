pub fn solve(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let nums = line
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>();
            let max1 = nums.iter().max().unwrap();
            let idx1 = nums
                .iter()
                .position(|val| *val == *max1)
                .unwrap();
            if idx1 < nums.len() - 1 {
                let max2 = nums.iter().skip(idx1 + 1).max().unwrap();
                return format!("{}{}", max1, max2).parse::<u32>().unwrap();
            } else {
                let max0 = nums.iter().take(idx1).max().unwrap();
                return format!("{}{}", max0, max1).parse::<u32>().unwrap();
            }
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../test.txt");
        assert_eq!(solve(input), 357);
    }
}
