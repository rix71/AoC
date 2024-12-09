use itertools::Itertools;

fn part1(disk_map: &str) {
    let mut extended_map: Vec<i64> = vec![];
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
    let mut idx_file;
    loop {
        idx_file = (extended_map.len() - 1)
            - extended_map
                .iter()
                .rev()
                .find_position(|x| x >= &&0)
                .unwrap()
                .0;
        let idx_slot = extended_map.iter().find_position(|x| x < &&0).unwrap().0;
        // println!("{:?} {:?}", idx_file, idx_slot);
        if idx_file <= idx_slot {
            break;
        }
        extended_map.swap(idx_file, idx_slot);
    }
    // println!("{:?}", extended_map);
    let checksum = extended_map
        .iter()
        .take(idx_file + 1)
        .enumerate()
        .map(|(i, &x)| x as usize * i)
        .sum::<usize>();
    println!("{:?}", checksum);
}

fn part2(disk_map: &str) {
    let mut extended_map: Vec<i64> = vec![];
    let mut file_info: Vec<(i64, i64)> = vec![];
    let mut slot_info: Vec<(i64, i64)> = vec![];
    let mut file_block = true;
    let mut file_id = 0;
    for c in disk_map.chars() {
        if file_block {
            let file_size = c.to_digit(10).unwrap();
            let idx = extended_map.len();
            file_info.push((file_size as i64, idx as i64));
            extended_map.extend(vec![file_id; file_size as usize]);
            file_id += 1;
            file_block = false;
        } else {
            let block_size = c.to_digit(10).unwrap();
            let idx = extended_map.len();
            slot_info.push((block_size as i64, idx as i64));
            extended_map.extend(vec![-1; block_size as usize]);
            file_block = true;
        }
    }
    while file_id > 0 {
        file_id -= 1;
        let (file_size, file_idx) = file_info[file_id as usize];
        for i in 0..slot_info.len() {
            if slot_info[i as usize].1 > file_idx {
                break;
            }
            if slot_info[i as usize].0 >= file_size {
                for j in 0..file_size {
                    extended_map.swap(
                        file_idx as usize + j as usize,
                        slot_info[i as usize].1 as usize + j as usize,
                    );
                }
                slot_info[i as usize].0 -= file_size;
                slot_info[i as usize].1 += file_size;
                break;
            }
        }
    }
    // println!("{:?}", extended_map);
    let checksum = extended_map
        .iter()
        .enumerate()
        .map(|(i, &x)| if x > 0 { x as usize * i } else { 0 })
        .sum::<usize>();
    println!("{:?}", checksum);
}

fn main() {
    let input = include_str!("../../input/day09/in.txt");
    println!("{:?}", input);
    part1(input);
    part2(input);
}
