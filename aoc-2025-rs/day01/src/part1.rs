pub fn solve(input: &str) -> u32 {
    let mut pos = 50;
    let mut total_at_zero = 0;
    input.lines().for_each(|line| {
        let dir = line.chars().nth(0).unwrap();
        let clicks = line[1..].parse::<i32>().unwrap();
        match dir {
            'L' => pos = (pos - clicks) % 100,
            'R' => pos = (pos + clicks) % 100,
            _ => panic!("Unknown direction"),
        }
        if pos == 0 {
            total_at_zero += 1;
        }
    });
    total_at_zero
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
