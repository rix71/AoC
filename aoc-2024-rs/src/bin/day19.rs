use std::collections::HashMap;

fn is_possible(
    design: &String,
    towels: &Vec<String>,
    towels_max: usize,
    cache: &mut HashMap<String, i64>,
) -> i64 {
    let mut count = 0;
    if design.is_empty() {
        return 1;
    }
    if cache.contains_key(design) {
        return *cache.get(design).unwrap() as i64;
    }
    for i in 0..design.len().min(towels_max) + 1 {
        if towels.contains(&design[..i].to_string()) {
            let npos = is_possible(&design[i..].to_string(), towels, towels_max, cache);
            count += npos
        }
    }
    cache.insert(design.to_string(), count);
    count
}

fn part1(towels: Vec<String>, designs: Vec<String>) -> i64 {
    let mut cache = HashMap::new();
    let towels_max = towels.iter().map(|t| t.len()).max().unwrap();
    designs
        .iter()
        .map(|design| is_possible(design, &towels, towels_max, &mut cache))
        .map(|x| if x > 0 { 1 } else { 0 })
        .sum()
}

fn part2(towels: Vec<String>, designs: Vec<String>) -> i64 {
    let mut cache = HashMap::new();
    let towels_max = towels.iter().map(|t| t.len()).max().unwrap();
    designs
        .iter()
        .map(|design| is_possible(design, &towels, towels_max, &mut cache))
        .sum()
}

fn main() {
    let input = include_str!("../../input/day19/in.txt");
    let (towels, designs) = input.split_once("\n\n").unwrap();
    let towels = towels
        .split(", ")
        .map(|t| t.to_string())
        .collect::<Vec<String>>();

    let designs = designs
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>();

    // println!("{:?}\n{:?}", towels, designs);
    println!("Part 1: {:?}", part1(towels.clone(), designs.clone()));
    println!("Part 2: {:?}", part2(towels.clone(), designs.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/day19/test.txt");
        let (towels, designs) = input.split_once("\n\n").unwrap();
        let towels = towels
            .split(", ")
            .map(|t| t.to_string())
            .collect::<Vec<String>>();

        let designs = designs
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();

        // println!("{:?}\n{:?}", towels, designs);

        assert_eq!(part1(towels, designs), 6);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/day19/test.txt");
        let (towels, designs) = input.split_once("\n\n").unwrap();
        let towels = towels
            .split(", ")
            .map(|t| t.to_string())
            .collect::<Vec<String>>();

        let designs = designs
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();
        
        assert_eq!(part2(towels, designs), 16);
    }
}
