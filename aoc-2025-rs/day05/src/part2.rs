pub fn solve(input: &str) -> u64 {
    let (ranges, _ids) = input.split_once("\n\n").unwrap();

    let mut ranges = ranges
        .lines()
        .map(|line| {
            line.split_once('-')
                .and_then(|(s, e)| Some((s.parse::<u64>().unwrap(), e.parse::<u64>().unwrap())))
                .unwrap()
        })
        .collect::<Vec<_>>();
    ranges.sort_by_key(|(start, _end)| *start);

    let mut merged_ranges = Vec::new();

    let mut i = 0;
    while i < ranges.len() - 1 {
        let (s1, e1) = ranges[i];
        let merged_start = s1;
        let mut merged_end = e1;
        for j in i + 1..ranges.len() {
            i = i + 1;
            let (s2, e2) = ranges[j];
            if merged_end + 1 < s2 {
                break;
            }
            merged_end = merged_end.max(e2);
        }
        merged_ranges.push((merged_start, merged_end));
    }

    merged_ranges.iter().map(|(s, e)| e - s + 1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = include_str!("../test.txt");
        assert_eq!(solve(input), 14);
    }
}
