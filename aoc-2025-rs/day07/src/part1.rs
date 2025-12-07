use std::collections::HashSet;

pub fn solve(input: &str) -> u64 {
    let mut lines_iter = input.lines();
    let start_pos = lines_iter
        .next()
        .map(|line| {
            println!("{line:?}");
            line.chars().position(|c| c == 'S').unwrap()
        })
        .unwrap();
    let mut beam_x = HashSet::new();
    beam_x.insert(start_pos);

    let mut splits = 0;
    lines_iter.for_each(|line| {
        for x in beam_x.clone().iter() {
            if line.chars().nth(*x).unwrap() == '^' {
                beam_x.remove(x);
                beam_x.insert(x - 1);
                beam_x.insert(x + 1);
                splits += 1;
            }
        }
    });
    splits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../test.txt");
        assert_eq!(solve(input), 21);
    }
}
