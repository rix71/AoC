use std::collections::HashMap;

use cached::SizedCache;
use cached::proc_macro::cached;

#[cached(
    ty = "SizedCache<String, u64>",
    create = "{ SizedCache::with_size(5000) }",
    convert = r#"{ format!("{}{}{}", dev, passed_fft, passed_dac) }"#
)]
fn find_path(
    dev: &String,
    devices: &HashMap<String, Vec<String>>,
    passed_fft: bool,
    passed_dac: bool,
) -> u64 {
    if dev == "out" {
        if passed_fft && passed_dac {
            return 1u64;
        } else {
            return 0u64;
        }
    }

    let path_has_fft = (dev == "fft") | passed_fft;
    let path_has_dac = (dev == "dac") | passed_dac;

    devices
        .get(dev)
        .unwrap()
        .iter()
        .map(|out| find_path(out, devices, path_has_fft, path_has_dac))
        .sum()
}

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

    let total = devices
        .get("svr")
        .unwrap()
        .iter()
        .map(|out| find_path(out, &devices, false, false))
        .sum();

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = include_str!("../test_part2.txt");
        assert_eq!(solve(input), 2);
    }
}
