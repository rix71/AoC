use ndarray::prelude::*;

pub fn solve(input: &str) -> u64 {
    let mut lines = input.lines().rev();
    let ops = lines
        .next()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .unwrap();

    let n_probs = ops.len();
    let num_vec = lines
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
        .iter()
        .cloned()
        .flatten()
        .collect::<Vec<_>>();
    let n_nums = num_vec.len() / n_probs;

    let probs = Array::from_vec(num_vec);
    let probs = probs.into_shape_with_order((n_nums, n_probs)).unwrap();

    let mut total = 0;
    for (op, c) in ops.iter().zip(probs.columns()) {
        let ans = match *op {
            "+" => c.sum(),
            "*" => c.product(),
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
    fn test_part1() {
        let input = include_str!("../test.txt");
        assert_eq!(solve(input), 4277556);
    }
}
