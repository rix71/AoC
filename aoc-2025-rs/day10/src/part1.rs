pub fn solve(input: &str) -> u64 {
    let lights_and_btns = input
        .lines()
        .map(|line| {
            let (lights, rest) = line.split_once(' ').unwrap();
            let lights = lights
                .strip_prefix('[')
                .unwrap()
                .strip_suffix(']')
                .unwrap()
                .chars()
                .map(|d| match d {
                    '.' => false,
                    '#' => true,
                    _ => panic!("What is this?"),
                })
                .collect::<Vec<_>>();
            let btns_and_joltage = rest.split_whitespace();
            let n = btns_and_joltage.clone().count();
            let buttons = btns_and_joltage
                .take(n - 1)
                .map(|b| {
                    b.strip_prefix('(')
                        .unwrap()
                        .strip_suffix(')')
                        .unwrap()
                        .split(',')
                        .map(|d| d.parse::<u64>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            (lights, buttons)
        })
        .collect::<Vec<_>>();

    let mut total_pushes = 0;
    for (target_lights, buttons) in lights_and_btns {
        let target_number = target_lights
            .iter()
            .enumerate()
            .fold(0, |acc, (i, b)| if *b { acc + (1 << i) } else { acc });

        let button_numbers = buttons
            .iter()
            .map(|btn| btn.iter().fold(0, |acc, d| acc + (1 << d)))
            .collect::<Vec<_>>();

        let mut least_pushes = 99999;
        for btn_conf in 0..1 << buttons.len() {
            let mut current = 0u64;
            let mut pushes = 0;
            for i in 0..buttons.len() {
                if (btn_conf >> i) % 2 == 1 {
                    current ^= button_numbers[i];
                    pushes += 1;
                }
            }
            if current == target_number {
                least_pushes = least_pushes.min(pushes);
            }
        }
        total_pushes += least_pushes;
    }

    total_pushes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../test.txt");
        assert_eq!(solve(input), 7);
    }
}
