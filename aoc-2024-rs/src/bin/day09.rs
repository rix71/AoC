fn part1(disk_map: &str) {
    let mut extended_map = vec![];
    let mut file_block = true;
    let mut file_id = 0;
    for c in disk_map.chars() {
        if file_block {
            let file_size = c.to_digit(10).unwrap();
            extended_map.extend(vec![file_id; file_size as usize]);
            file_id += 1;
            file_block = false;
        } else {
            let block_size = c.to_digit(10).unwrap();
            extended_map.extend(vec![-1; block_size as usize]);
            file_block = true;
        }
    }
    let (sorted, _): (Vec<i32>, Vec<i32>) = extended_map.iter().partition(|x| x >= &&0); // Nope!
    println!("{:?}", sorted);
    let checksum = sorted
        .iter()
        .enumerate()
        .map(|(i, &x)| x as usize * i)
        .sum::<usize>();
    println!("{:?}", checksum);
}

fn main() {
    let input = include_str!("../../input/day09/test.txt");
    println!("{:?}", input);
    part1(input);
}
