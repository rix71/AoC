use std::collections::HashMap;

fn part1(mut l1: Vec<i32>, mut l2: Vec<i32>) {
    l1.sort();
    l2.sort();
    let result: i32 = l1.iter().zip(l2.iter()).map(|(a, b)| (a - b).abs()).sum();
    println!("{result}");
}

fn part2(l1: Vec<i32>, l2: Vec<i32>) {
    let l2_counts = l2.iter().fold(HashMap::new(), |mut acc, &x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    });
    let result: i32 = l1.iter().map(|x| x * l2_counts.get(x).unwrap_or(&0)).sum();
    println!("{result}");
}

fn main() {
    let input = include_str!("../../input/day01/test.txt");
    let (l1, l2) = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .fold((vec![], vec![]), |(mut l1, mut l2), l| {
            l1.push(l[0]);
            l2.push(l[1]);
            (l1, l2)
        });

    part1(l1.clone(), l2.clone());
    part2(l1.clone(), l2.clone());
}
