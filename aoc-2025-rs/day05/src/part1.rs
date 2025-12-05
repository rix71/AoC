pub fn solve(input: &str) -> u64 {
    let (ranges, ids) = input.split_once("\n\n").unwrap();

    let ranges = ranges
        .lines()
        .map(|line| {
            line.split_once('-')
                .and_then(|(s, e)| Some((s.parse::<u64>().unwrap(), e.parse::<u64>().unwrap())))
                .unwrap()
        })
        .collect::<Vec<_>>();
    
    ids.lines()
        .map(|line| {
            let id = line.parse::<u64>().unwrap();
            for (start, end) in &ranges {
                if id >= *start && id <= *end {
                    return 1;
                }
            }
            0
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../test.txt");
        assert_eq!(solve(input), 3);
    }
}
