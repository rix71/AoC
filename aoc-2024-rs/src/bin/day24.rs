use std::{collections::HashMap, mem::swap};

use itertools::Itertools;

fn parse(
    input: &str,
) -> (
    HashMap<String, Option<u32>>,
    Vec<(String, String, String, String)>,
) {
    let (wire_values, gates) = input.split_once("\n\n").unwrap();
    let initial_wires = wire_values
        .lines()
        .map(|line| {
            let (wire_name, wire_value) = line.split_once(": ").unwrap();
            let wire_value = wire_value.parse().unwrap();
            (wire_name, wire_value)
        })
        .fold(HashMap::new(), |mut acc, (wn, wv)| {
            acc.insert(wn, wv);
            acc
        });

    let gates = gates
        .lines()
        .map(|line| {
            let cont = line.split_whitespace().collect::<Vec<&str>>();
            assert_eq!(cont.len(), 5);
            let w1 = cont[0].to_string();
            let w2 = cont[2].to_string();
            let op = cont[1].to_string();
            let out = cont[4].to_string();
            (w1, w2, op, out)
        })
        .collect::<Vec<_>>();

    let mut wires = HashMap::<String, Option<u32>>::new();
    gates.iter().for_each(|(w1, w2, _op, out)| {
        wires.insert(w1.clone(), initial_wires.get(w1.as_str()).copied());
        wires.insert(w2.clone(), initial_wires.get(w2.as_str()).copied());
        wires.insert(out.clone(), initial_wires.get(out.as_str()).copied());
    });

    (wires, gates)
}

fn solve_system(
    mut wires: HashMap<String, Option<u32>>,
    gates: Vec<(String, String, String, String)>,
) -> HashMap<String, Option<u32>> {
    let all_z_wires_have_values = |wires: &HashMap<String, Option<u32>>| {
        wires
            .iter()
            .filter(|(k, _)| k.starts_with("z"))
            .all(|(_, v)| v.is_some())
    };

    while !all_z_wires_have_values(&wires) {
        for (w1, w2, op, out) in gates.iter() {
            let w1 = wires.get(w1).unwrap();
            let w2 = wires.get(w2).unwrap();
            if w1.is_none() || w2.is_none() {
                continue;
            }
            match op.as_str() {
                "AND" => {
                    let res = Some(w1.unwrap() & w2.unwrap());
                    wires.entry(out.clone()).and_modify(|v| *v = res);
                }
                "OR" => {
                    let res = Some(w1.unwrap() | w2.unwrap());
                    wires.entry(out.clone()).and_modify(|v| *v = res);
                }
                "XOR" => {
                    let res = Some(w1.unwrap() ^ w2.unwrap());
                    wires.entry(out.clone()).and_modify(|v| *v = res);
                }
                _ => panic!("Unknown operation: {}", op),
            }
        }
    }
    wires
}

fn part1(wires: HashMap<String, Option<u32>>, gates: Vec<(String, String, String, String)>) -> u64 {
    let wires = solve_system(wires, gates);
    let ans = wires
        .iter()
        .filter(|(k, _)| k.starts_with("z"))
        .sorted()
        .rev()
        .fold(0, |acc, (_, v)| (acc << 1) | v.unwrap() as u64);
    ans
}

fn part2(
    wires: HashMap<String, Option<u32>>,
    gates: Vec<(String, String, String, String)>,
    expected_op: impl Fn(u64, u64) -> u64,
    expected_len: usize,
) -> String {
    let x = wires
        .iter()
        .filter(|(k, _)| k.starts_with("x"))
        .sorted()
        .rev()
        .fold(0, |acc, (_, v)| (acc << 1) | v.unwrap() as u64);
    let y = wires
        .iter()
        .filter(|(k, _)| k.starts_with("y"))
        .sorted()
        .rev()
        .fold(0, |acc, (_, v)| (acc << 1) | v.unwrap() as u64);

    let bit_count = wires.iter().filter(|(k, _)| k.starts_with("z")).count();

    let z = expected_op(x, y);

    println!("-----------------");
    println!("x: {:048b}\ny: {:048b}\nz: {:048b}", x, y, z);
    println!("x: {}\ny: {}\nz: {}", x, y, z);

    let wires_final = solve_system(wires.clone(), gates.clone());

    let z_final = wires_final
        .iter()
        .filter(|(k, _)| k.starts_with("z"))
        .sorted()
        .rev()
        .fold(0, |acc, (_, v)| (acc << 1) | v.unwrap() as u64);

    println!("-----------------");
    println!("z: {:048b}", z);
    println!("z: {:048b}", z_final);
    let mask = (1_u64 << 46) - 1;
    let zxnor = !(z ^ z_final) & mask;
    println!("z: {:0>48}", format!("{:b}", zxnor));
    println!("z: {}", z_final);

    let mut swap_cands = Vec::new();
    for i in 0..bit_count {
        if (zxnor >> i) & 1 == 0 {
            swap_cands.push(i);
        }
    }
    println!("swap_cands: {:?}", swap_cands);

    println!("-----------------");
    // println!("Gates before swap: {:?}", gates);

    let max_skips = swap_cands.len() - expected_len;


    for n_skip in 0..max_skips + 1 {
        let mut fixed_gates = gates.clone();

        for chunks in &swap_cands.iter().skip(n_skip).take(expected_len).chunks(2) {
            let c = chunks.collect::<Vec<_>>();
            let a = c[0];
            let b = c[1];
            let (idx_a, &ref sa) = fixed_gates
                .iter()
                .find_position(|(_w1, _w2, _op, out)| *out == format!("z{:02}", a))
                .unwrap();
            let (idx_b, &ref sb) = fixed_gates
                .iter()
                .find_position(|(_w1, _w2, _op, out)| *out == format!("z{:02}", b))
                .unwrap();
            // println!("Swapping {:?} and {:?}", sa, sb);
            let mut sa_copy = sa.clone();
            let mut sb_copy = sb.clone();
            swap(&mut sa_copy.3, &mut sb_copy.3);
            // println!("Swapped {:?} and {:?}", sa_copy, sb_copy);
            fixed_gates[idx_a] = sa_copy;
            fixed_gates[idx_b] = sb_copy;
        }
        // println!("Gates after swap: {:?}", fixed_gates);

        let wires_final = solve_system(wires.clone(), fixed_gates.clone());

        let z_final = wires_final
            .iter()
            .filter(|(k, _)| k.starts_with("z"))
            .sorted()
            .rev()
            .fold(0, |acc, (_, v)| (acc << 1) | v.unwrap() as u64);

        println!("-----------------");
        println!("z: {:048b}", z);
        println!("z: {:048b}", z_final);
        let mask = (1_u64 << 46) - 1;
        let zxnor = !(z ^ z_final) & mask;
        println!("z: {:0>48}", format!("{:b}", zxnor));
        println!("z: {}", z_final);
    }

    "".into()
}

fn main() {
    let input = include_str!("../../input/day24/in.txt");
    let (initial_wires, gates) = parse(input);

    let result = part1(initial_wires.clone(), gates.clone());
    println!("Part 1: {}", result);

    let result = part2(initial_wires, gates, |x, y| x + y, 8);
    println!("Part 2: {}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1_part1() {
        use super::*;
        let input = include_str!("../../input/day24/test1.txt");
        let (initial_wires, gates) = parse(input);
        let result = part1(initial_wires, gates);
        assert_eq!(result, 4);
    }

    #[test]
    fn test2_part1() {
        use super::*;
        let input = include_str!("../../input/day24/test2.txt");
        let (initial_wires, gates) = parse(input);
        let result = part1(initial_wires, gates);
        assert_eq!(result, 2024);
    }

    #[test]
    fn test1_part2() {
        use super::*;
        let input = include_str!("../../input/day24/test3.txt");
        let (initial_wires, gates) = parse(input);
        let result = part2(initial_wires, gates, |x, y| x & y, 2);
        assert_eq!(result, "z00,z01,z02,z05");
    }
}
