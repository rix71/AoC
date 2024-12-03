use itertools::Itertools;

fn part1(reports: Vec<Vec<i32>>) {
    let result: i32 = reports
        .iter()
        .map(|report| if is_safe(report) { 1 } else { 0 })
        .sum();
    println!("{result}");
}

fn part2(reports: Vec<Vec<i32>>) {
    let result: i32 = reports
        .iter()
        .map(|report| {
            if is_safe(report) {
                return 1;
            }
            for i in 0..report.len() {
                let mut new_report = report.clone();
                new_report.remove(i);
                if is_safe(&new_report) {
                    return 1;
                }
            }
            return 0;
        })
        .sum();
    println!("{result}");
}

fn is_safe(report: &[i32]) -> bool {
    let diffs: Vec<i32> = report.iter().tuple_windows().map(|(a, b)| a - b).collect();
    (diffs.iter().all(|diff| diff > &0) || diffs.iter().all(|diff| diff < &0))
        && diffs.iter().all(|diff| diff.abs() >= 1 && diff.abs() <= 3)
}

fn main() {
    let input = include_str!("../../input/day02/in.txt");
    // println!("input: {}", input);
    let reports: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect();

    part1(reports.clone());
    part2(reports.clone());
}
