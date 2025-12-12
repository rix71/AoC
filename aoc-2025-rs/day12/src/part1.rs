fn parse(input: &str) -> (Vec<Vec<Vec<usize>>>, Vec<((usize, usize), Vec<usize>)>) {
    let sections: Vec<&str> = input.split("\n\n").collect();
    let n_sections = sections.len();
    let shapes = sections[..n_sections - 1]
        .iter()
        .map(|s| {
            let shape = s
                .lines()
                .skip(1)
                .map(|l| {
                    l.chars()
                        .map(|c| if c == '#' { 1usize } else { 0usize })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            shape
        })
        .collect::<Vec<_>>();
    let regions = sections[n_sections - 1]
        .lines()
        .map(|l| {
            let (size, presents) = l.split_once(": ").unwrap();
            let size = size
                .split_once('x')
                .map(|(w, h)| (w.parse::<usize>().unwrap(), h.parse::<usize>().unwrap()))
                .unwrap();
            let presents = presents
                .split_whitespace()
                .map(|p| p.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            (size, presents)
        })
        .collect::<Vec<_>>();
    (shapes, regions)
}

pub fn solve(input: &str) -> usize {
    let (shapes, regions) = parse(input);
    println!("{:?}", shapes);
    println!("{:?}", regions);

    let mut total = 0;
    for ((w, h), presents) in regions {
        let region_area = w * h;
        let required_area = presents
            .iter()
            .zip(shapes.iter())
            .map(|(p, s)| {
                if *p > 0 {
                    let shape_area = s.iter().flatten().sum::<usize>();
                    println!("shape idx {} area: {}", *p, shape_area);
                    return *p * shape_area;
                }
                0
            })
            .sum::<usize>();

        println!(
            "region area: {}, required area: {}",
            region_area, required_area
        );

        if region_area >= required_area {
            total += 1;
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../test.txt");
        assert_eq!(solve(input), 2);
    }
}
