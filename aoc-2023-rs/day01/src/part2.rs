pub fn solve(input: &str) -> u32 {
    let digits = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    input
        .lines()
        .map(|line| {
            let scan_line = |idx: usize| {
                if line.chars().nth(idx).unwrap().is_numeric() {
                    return Some(line.chars().nth(idx).unwrap().to_digit(10).unwrap());
                } else {
                    for (i, digit) in digits.iter().enumerate() {
                        if line.get(idx..).unwrap_or_default().starts_with(digit) {
                            return Some(i as u32 + 1);
                        }
                    }
                    None
                }
            };
            let mut front_idx = 0;
            let mut back_idx = line.len() - 1;
            let mut front_val: Option<u32> = None;
            let mut back_val: Option<u32> = None;
            while front_val.is_none() || back_val.is_none() {
                if front_val.is_none() {
                    front_val = scan_line(front_idx);
                    front_idx += 1;
                }
                if back_val.is_none() {
                    back_val = scan_line(back_idx);
                    if back_val.is_none() {
                        back_idx -= 1;
                    }
                }
            }
            front_val.unwrap() * 10 + back_val.unwrap()
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = include_str!("../test_part2.txt");
        let result = solve(input);
        assert_eq!(result, 281);
    }
}
