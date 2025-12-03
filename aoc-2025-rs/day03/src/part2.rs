pub fn solve(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let nums = line
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>();
            let mut jolt_vec = vec![0u32; 12];
            let mut idx_start = 0;
            for idx in 1..=12 {
                let slice = &nums[idx_start..nums.len() - (12 - idx)];
                let slice_max = slice.iter().max().unwrap();
                let idx_slice_max = slice.iter().position(|val| *val == *slice_max).unwrap();
                idx_start = idx_start + idx_slice_max + 1;
                jolt_vec[idx - 1] = *slice_max;
            }
            jolt_vec
                .iter()
                .fold(String::new(), |s, val| s + &val.to_string())
                .parse::<u64>()
                .unwrap()
        })
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = include_str!("../test.txt");
        assert_eq!(solve(input), 3121910778619);
    }
}
