use regex::Regex;

fn sum_instructions(memory: &str) -> u32 {
    let re = Regex::new(r"mul\((\d{1,3},\d{1,3})\)").unwrap();
    let result: u32 = re
        .captures_iter(&memory)
        .map(|c| {
            let (_, [nums]) = c.extract();
            nums.split(",")
                .map(|n| n.parse::<u32>().unwrap())
                .product::<u32>()
        })
        .sum();
    result
}

fn part1(memory: &str) {
    println!("{:?}", sum_instructions(memory));
}

fn part2(memory: &str) {
    let mut enabled = true;
    let mut total = 0;
    memory.split("don't()").for_each(|m| {
        if enabled {
            total += sum_instructions(m);
            enabled = false;
        } else {
            total += m
                .split("do()")
                .into_iter()
                .skip(1)
                .map(|m| sum_instructions(m))
                .sum::<u32>();
        }
    });
    println!("{:?}", total);
}

fn main() {
    let input = include_str!("../../input/day3/in.txt");
    // println!("{}", input);
    part1(input);
    part2(input);
}
