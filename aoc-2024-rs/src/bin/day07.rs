use itertools::Itertools;

fn could_be_true(target: &u64, values: &Vec<u64>, available_ops: Vec<&str>) -> bool {
    println!("Checking {:?} with {:?}", target, values);
    if values.iter().sum::<u64>() == *target {
        return true;
    }
    if values.iter().product::<u64>() == *target {
        return true;
    }

    let ops = vec![available_ops; values.len() - 1];

    for mcp in ops.iter().multi_cartesian_product() {
        let res = values[1..]
            .iter()
            .zip(mcp.iter())
            .fold(values[0], |acc, (v, &&op)| match op {
                "+" => acc + v,
                "*" => acc * v,
                "||" => format!("{}{}", acc, v).parse::<u64>().unwrap(),
                _ => panic!("Unknown operator {:?}", op),
            });
        if res == *target {
            return true;
        }
    }
    false
}

fn part1(targets_list: Vec<u64>, values_list: Vec<Vec<u64>>) {
    let result = targets_list
        .iter()
        .zip(values_list.iter())
        .filter(|(target, values)| could_be_true(target, values, vec!["+", "*"]))
        .map(|(target, _)| target)
        .sum::<u64>();
    println!("{:?}", result);
}

fn part2(targets_list: Vec<u64>, values_list: Vec<Vec<u64>>) {
    let result = targets_list
        .iter()
        .zip(values_list.iter())
        .filter(|(target, values)| could_be_true(target, values, vec!["+", "*", "||"]))
        .map(|(target, _)| target)
        .sum::<u64>();

    println!("{:?}", result);
}

fn main() {
    let input = include_str!("../../input/day07/in.txt");

    let (targets, values) = input
        .lines()
        .map(|line| {
            let split_line: Vec<&str> = line.split(": ").collect();
            let target = split_line.first().unwrap().parse::<u64>().unwrap();
            let vals = split_line
                .iter()
                .nth(1)
                .unwrap()
                .split(" ")
                .map(|v| v.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            (target, vals)
        })
        .fold((vec![], vec![]), |(mut tv, mut vv), (t, v)| {
            tv.push(t);
            vv.push(v);
            (tv, vv)
        });

    println!("{:?}", targets);
    println!("{:?}", values);

    part1(targets.clone(), values.clone());
    part2(targets.clone(), values.clone());
}
