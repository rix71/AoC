use std::collections::HashSet;

use glam::Vec3;

#[cfg(test)]
const NPAIRS: usize = 10;

#[cfg(not(test))]
const NPAIRS: usize = 1000;

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

    let mut seen = HashSet::new();
    for (_dist, (p1, p2)) in distances.iter() {
        let p1c = circuits.iter().find(|c| c.contains(p1)).cloned().unwrap();
        let p2c = circuits.iter().find(|c| c.contains(p2)).cloned().unwrap();
        let merged = p1c.union(&p2c).cloned().collect::<HashSet<_>>();
        circuits.retain(|c| *c != p1c && *c != p2c);
        circuits.push(merged);

        seen.insert((*p1, *p2));
        if seen.len() == NPAIRS {
            break;
        }
    }

    circuits.sort_by_key(|j| j.len());
    circuits.reverse();
    circuits[..3].iter().map(|j| j.len() as u64).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../test.txt");
        assert_eq!(solve(input), 40);
    }
}
