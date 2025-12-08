use std::collections::HashSet;

use glam::Vec3;

pub fn solve(input: &str) -> u64 {
    let positions = input
        .lines()
        .map(|line| {
            let pos = line
                .split(',')
                .map(|d| d.parse().unwrap())
                .collect::<Vec<_>>();
            Vec3::from_slice(pos.as_slice())
        })
        .collect::<Vec<_>>();

    let mut distances = Vec::new();
    for i in 0..positions.len() {
        for j in i + 1..positions.len() {
            let dist = positions[i].distance(positions[j]);
            distances.push((dist, (i, j)));
        }
    }

    distances.sort_by(|d1, d2| d1.0.total_cmp(&d2.0));

    let mut circuits = Vec::<HashSet<_>>::new();
    for i in 0..positions.len() {
        circuits.push(HashSet::from([i]));
    }

    let mut hit1 = 0;
    let mut hit2 = 0;
    for (_dist, (p1, p2)) in distances.iter() {
        let p1c = circuits.iter().find(|c| c.contains(p1)).cloned().unwrap();
        let p2c = circuits.iter().find(|c| c.contains(p2)).cloned().unwrap();
        let merged = p1c.union(&p2c).cloned().collect::<HashSet<_>>();
        circuits.retain(|c| *c != p1c && *c != p2c);
        circuits.push(merged);

        if circuits.len() == 1 {
            hit1 = positions[*p1].x as u64;
            hit2 = positions[*p2].x as u64;
            break;
        }
    }

    hit1 * hit2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = include_str!("../test.txt");
        assert_eq!(solve(input), 25272);
    }
}
