use std::collections::HashMap;

pub fn solve(input: &str) -> u64 {
    let devices = input.lines().fold(HashMap::new(), |mut acc, line| {
        let s = line
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let device = s[0][..3].to_string();
        let outputs = s[1..].into_iter().cloned().collect::<Vec<_>>();

        acc.insert(device, outputs);
        acc
    });

    let mut outputs = devices.get("you").unwrap().clone();
    let mut total = 0;
    while let Some(dev) = outputs.pop() {
        if dev == "out" {
            total += 1;
            continue;
        }

        outputs.extend_from_slice(devices.get(&dev).unwrap());
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../test.txt");
        assert_eq!(solve(input), 5);
    }
}
