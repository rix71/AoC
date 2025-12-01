pub fn solve(input: &str) -> u32 {
    let mut pos = 50;
    let mut total_at_zero = 0;
    input.lines().for_each(|line| {
        let dir = line.chars().nth(0).unwrap();
        let clicks = line[1..].parse::<i32>().unwrap();
        for _ in 0..clicks {
            match dir {
                'L' => pos = (pos - 1 + 100) % 100,
                'R' => pos = (pos + 1) % 100,
                _ => panic!("Unknown direction"),
            }
            if pos == 0 {
                total_at_zero += 1;
            }
        }
    });
    total_at_zero
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = include_str!("../test.txt");
        assert_eq!(solve(input), 6);
    }
}
