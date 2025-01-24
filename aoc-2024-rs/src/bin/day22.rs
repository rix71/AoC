use std::collections::{HashMap, HashSet};

fn sequence(n: i64) -> i64 {
    let mut n = ((n << 6) ^ n) % 16777216;
    n = ((n >> 5) ^ n) % 16777216;
    ((n << 11) ^ n) % 16777216
}

fn part1(initial: Vec<i64>) -> i64 {
    let mut ans = 0;
    for n in initial.iter() {
        let mut new_n = *n;
        for _ in 0..2000 {
            new_n = sequence(new_n);
        }
        ans += new_n;
    }
    ans
}

fn part2(initial: Vec<i64>) -> i64 {
    *initial
        .iter()
        .map(|n| {
            let mut n = *n;
            let mut seq = vec![];
            let mut changes = vec![];
            for _ in 0..2000 {
                seq.push(n);
                let new_n = sequence(n);
                changes.push((new_n % 10) - (n % 10));
                n = new_n;
            }
            seq.push(n);
            (seq, changes)
        })
        .fold(HashMap::new(), |mut acc, (seq, changes)| {
            let mut seen = HashSet::new();
            for i in 0..(2000 - 3) {
                let chsub = (changes[i], changes[i + 1], changes[i + 2], changes[i + 3]);
                if !seen.contains(&chsub) {
                    acc.entry(chsub.clone())
                        .and_modify(|e| *e += seq[i + 4] % 10)
                        .or_insert(seq[i + 4] % 10);
                    seen.insert(chsub);
                }
            }
            acc
        })
        .values()
        .max()
        .unwrap()
}

fn main() {
    let input = include_str!("../../input/day22/in.txt");
    let initial: Vec<i64> = input
        .lines()
        .into_iter()
        .map(|line| line.parse().unwrap())
        .collect();

    let start = std::time::Instant::now();
    println!("Part 1: {:?}", part1(initial.clone()));
    let elapsed = start.elapsed().as_micros();
    println!("Elapsed: {}µs", elapsed);

    let start = std::time::Instant::now();
    println!("Part 2: {:?}", part2(initial));
    let elapsed = start.elapsed().as_micros();
    println!("Elapsed: {}µs", elapsed);
}
