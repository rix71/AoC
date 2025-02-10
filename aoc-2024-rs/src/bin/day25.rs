use itertools::Itertools;
use ndarray::{Array2, Axis};

macro_rules! measure {
    ($x:expr) => {
        let start = std::time::Instant::now();
        $x;
        let end = std::time::Instant::now();
        let elapsed = end - start;
        println!("{} took {:?} µs", stringify!($x), elapsed.as_micros());
    };
}

type Schema = Array2<i32>;

fn part1(keys: Vec<Schema>, locks: Vec<Schema>) {
    let h = keys[0].shape()[0];

    let lock_heights = locks
        .iter()
        .map(|l| l.sum_axis(Axis(0)) - 1)
        .collect::<Vec<_>>();
    let key_heights = keys
        .iter()
        .map(|k| k.sum_axis(Axis(0)) - 1)
        .collect::<Vec<_>>();

    let mut matches = 0;
    for (l, k) in lock_heights.iter().cartesian_product(key_heights.iter()) {
        let s = l + k;
        if s.iter().all(|x| *x < h as i32 - 1) {
            matches += 1;
        }
    }

    println!("matches: {:?}", matches);
}

fn main() {
    let input = include_str!("../../input/day25/in.txt");
    let (keys, locks) = input
        .split("\n\n")
        .map(|schem| {
            schem
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|x| if x == '#' { 1 } else { 0 })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .fold(
            (Vec::<Schema>::new(), Vec::<Schema>::new()),
            |(mut keys, mut locks), schema| {
                let h = schema.len();
                let w = schema[0].len();
                let a =
                    Array2::from_shape_vec((h, w), schema.into_iter().flatten().collect()).unwrap();
                if a.row(0).iter().all(|&x| x == 0) {
                    keys.push(a);
                } else {
                    locks.push(a);
                }
                (keys, locks)
            },
        );

    measure!(part1(keys, locks));
}
