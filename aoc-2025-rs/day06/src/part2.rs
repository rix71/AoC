use std::vec;

use ndarray::prelude::*;

pub fn solve(input: &str) -> u64 {
    let mut lines = input.lines().rev();
    let ops = lines
        .next()
        .map(|line| {
            let mut ops = Vec::new();
            let mut col_idx = 0;
            for c in line.chars() {
                if !c.is_whitespace() {
                    ops.push((c.to_string(), col_idx));
                }
                col_idx += 1;
            }
            ops
        })
        .unwrap();

    let num_vec = lines
        .map(|line| {
            // let l_str = line.to_string();
            let mut nums = Vec::new();
            let mut s = 0;
            for c in ops[1..].iter() {
                let e = c.1;
                nums.push(line.chars().skip(s).take(e - s - 1).collect::<String>());
                s = e;
            }
            nums.push(line.chars().skip(s).collect::<String>());
            nums
        })
        .flatten()
        .collect::<Vec<_>>();

    let n_probs = ops.len();
    let n_nums = num_vec.len() / n_probs;

    let probs = Array::from_vec(num_vec);
    let probs = probs.into_shape_with_order((n_nums, n_probs)).unwrap();

    let mut total = 0;
    for (op, c) in ops.iter().zip(probs.columns()) {
        let max_len = c.map(|ns| ns.len()).iter().max().unwrap().clone();
        let mut p10 = vec![1u64; max_len];
        let mut nums = vec![0u64; max_len];
        c.for_each(|ns| {
            for (i, d) in ns.chars().enumerate() {
                if d.is_digit(10) {
                    nums[i] += d.to_digit(10).unwrap() as u64 * p10[i];
                    p10[i] *= 10;
                }
            }
        });
        let ans = match op.0.as_str() {
            "+" => nums.iter().sum::<u64>(),
            "*" => nums.iter().product::<u64>(),
            _ => panic!("Unknown operation"),
        };
        total += ans;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = include_str!("../test.txt");
        assert_eq!(solve(input), 3263827);
    }
}
