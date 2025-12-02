use itertools::Itertools;

pub fn solve(input: &str) -> u64 {
    input
        .split(',')
        .map(|range| {
            let (start, end) = range.split_once("-").unwrap();
            let start = start.parse::<u64>().unwrap();
            let end = end.parse::<u64>().unwrap();

            (start..=end)
                .map(|num| {
                    if num.ilog10() < 1 {
                        return 0;
                    }

                    let digits = num.to_string().chars().collect::<Vec<_>>();

                    if digits.iter().all_equal() {
                        return num;
                    }

                    for chunk_size in 2..=digits.len() / 2 {
                        if digits.len() % chunk_size != 0 {
                            continue;
                        }
                        if digits
                            .iter()
                            .chunks(chunk_size)
                            .into_iter()
                            .map(|c| c.collect::<Vec<_>>())
                            .all_equal()
                        {
                            return num;
                        }
                    }
                    0
                })
                .sum::<u64>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = include_str!("../test.txt");
        assert_eq!(solve(input), 4174379265);
    }
}
