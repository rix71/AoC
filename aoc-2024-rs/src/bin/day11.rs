use std::collections::HashMap;

type Stones = HashMap<u64, u64>;

fn apply_rules(stones: &Stones) -> Stones {
    let mut new_values = Stones::new();
    stones.iter().for_each(|(value, ammount)| {
        if *value == 0 {
            new_values
                .entry(1)
                .and_modify(|e| *e += *ammount)
                .or_insert(*ammount);
        } else {
            let num_digits = value.checked_ilog10().unwrap_or(0) + 1;
            if num_digits % 2 == 0 {
                let value_str = value.to_string();
                let (first, second) = value_str.split_at(num_digits as usize / 2);
                let first = first.parse::<u64>().unwrap();
                let second = second.parse::<u64>().unwrap();
                new_values
                    .entry(first)
                    .and_modify(|e| *e += *ammount)
                    .or_insert(*ammount);
                new_values
                    .entry(second)
                    .and_modify(|e| *e += *ammount)
                    .or_insert(*ammount);
            } else {
                new_values
                    .entry(value * 2024)
                    .and_modify(|e| *e += *ammount)
                    .or_insert(*ammount);
            }
        }
    });
    new_values
}

fn part1(stones: Stones) {
    let mut stones = stones;
    for _ in 0..25 {
        stones = apply_rules(&stones);
    }
    let total = stones.values().sum::<u64>();
    println!("{:?}", total);
}

fn part2(stones: Stones) {
    let mut stones = stones;
    for _ in 0..75 {
        stones = apply_rules(&stones);
    }
    let total = stones.values().sum::<u64>();
    println!("{:?}", total);
}

fn main() {
    let input = include_str!("../../input/day11/in.txt");

    let stones = input.split_whitespace().fold(HashMap::new(), |mut acc, d| {
        acc.entry(d.parse::<u64>().unwrap())
            .and_modify(|e| *e += 1)
            .or_insert(1);
        acc
    });

    println!("{:?}", stones);

    part1(stones.clone());
    part2(stones.clone());
}
