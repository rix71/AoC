use itertools::Itertools;

pub fn solve(input: &str) -> u64 {
    input
        .split(",")
        .map(|range| {
            let (start, end) = range.split_once("-").unwrap();
            let start = start.parse::<u64>().unwrap();
            let end = end.parse::<u64>().unwrap();

            let mut id_sum = 0;

            for num in start..=end {
                println!("  Number: {num}");
                if (num.ilog10() + 1) % 2 != 0 {
                    continue;
                }
                let digits = num.to_string().chars().collect::<Vec<_>>();
                if digits.iter().all_equal() {
                    id_sum += num;
                } else if itertools::equal(
                    digits[..digits.len() / 2].iter(),
                    digits[digits.len() / 2..].iter(),
                ) {
                    id_sum += num;
                }
            }
            id_sum
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../test.txt");
        assert_eq!(solve(input), 1227775554);
    }
}
