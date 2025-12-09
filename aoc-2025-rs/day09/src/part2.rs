use glam::U64Vec2;
use itertools::Itertools;

fn corner_index12(pl: &U64Vec2, pc: &U64Vec2, pr: &U64Vec2) -> Option<(i8, i8)> {
    if pc.x < pr.x && pc.y == pr.y && pc.y < pl.y && pc.x == pl.x {
        // #--pr
        // |
        // pl
        Some((1, 1))
    } else if pc.x > pl.x && pc.y == pl.y && pc.y < pr.y && pc.x == pr.x {
        // pl--#
        //     |
        //    pr
        Some((-1, 1))
    } else if pc.x < pr.x && pc.y == pr.y && pc.y > pl.y && pc.x == pl.x {
        // pl
        // |
        // #--pr
        Some((1, -1))
    } else if pc.x > pl.x && pc.y == pl.y && pc.y > pr.y && pc.x == pr.x {
        //     pr
        //     |
        // pl--#
        Some((-1, -1))
    } else {
        // dbg!(pc.x > pl.x);
        None
    }
}

fn corner_index(pl: &U64Vec2, pc: &U64Vec2, pr: &U64Vec2) -> (i8, i8) {
    let has_neighbor_left = pc.x > pl.x || pc.x > pr.x;
    let has_neighbor_up = pc.y > pl.y || pc.y > pr.y;
    match (has_neighbor_left, has_neighbor_up) {
        (true, true) => (-1, -1),
        (true, false) => (-1, 1),
        (false, true) => (1, -1),
        (false, false) => (1, 1),
    }
}

pub fn solve12(input: &str) -> u64 {
    let positions = input
        .lines()
        .map(|line| {
            U64Vec2::from(
                line.split_once(',')
                    .map(|(x, y)| (x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap()))
                    .unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let max_area = positions
        .iter()
        .circular_tuple_windows()
        .map(|(p1, p2, p3, p4, p5)| {
            let ci2 = corner_index(p1, p2, p3);
            let ci3 = corner_index(p2, p3, p4);
            let ci4 = corner_index(p3, p4, p5);
            if ci2.0 == -1 * ci4.0 || ci2.1 == -1 * ci4.1 {
            // if (p4.x.max(p5.x) - p4.x.min(p5.x)) >= (p2.x.max(p3.x) - p2.x.min(p3.x)){
                let dx = (p2.x.max(p4.x) - p2.x.min(p4.x)) + 1;
                let dy = (p2.y.max(p4.y) - p2.y.min(p4.y)) + 1;
                let area = dx * dy;
                println!(
                    "Points: {:?}, {:?}, {:?}, {:?}, {:?}; Corner Indices: {:?}, {:?}, {:?} => Area: {} x {} = {}",
                    p1, p2, p3, p4, p5, ci2, ci3, ci4, dx, dy, area
                );
                assert!(area > 0);
                area
            // } else {
            //         0
            //     }
            } else {
                0
            }
        })
        .max()
        .unwrap();
    // .collect::<Vec<_>>();

    max_area
}

pub fn solve(input: &str) -> u64 {
    let positions = input
        .lines()
        .map(|line| {
            U64Vec2::from(
                line.split_once(',')
                    .map(|(x, y)| (x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap()))
                    .unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let max_area = positions
        .iter()
        .circular_tuple_windows()
        .map(|(p1, p2, p3, p4, p5)| {
            let ci2 = corner_index(p1, p2, p3);
            let ci3 = corner_index(p2, p3, p4);
            let ci4 = corner_index(p3, p4, p5);

            match ci3 {
                (1, 1) => {
                    println!("{:?} is (1,1)", p3);
                    let lb = positions
                        .iter()
                        .find(|p| p.x == p3.x && p.y > p3.y)
                        .expect("Didn't find lb");
                    if let Some(rb) = positions.iter().find(|p| p.x < lb.x && p.y == lb.y) {
                        let dx = (p3.x.max(rb.x) - p3.x.min(rb.x)) + 1;
                        let dy = (p3.y.max(lb.y) - p3.y.min(lb.y)) + 1;
                        let area = dx * dy;
                        area
                    } else {
                        0
                    }
                }
                (-1, 1) => {
                    println!("{:?} is (-1,1)", p3);
                    let rb = positions
                        .iter()
                        .find(|p| p.x == p3.x && p.y > p3.y)
                        .expect("Didn't find rb");
                    if let Some(lb) = positions.iter().find(|p| p.x < rb.x && p.y == rb.y) {
                        let dx = (p3.x.max(lb.x) - p3.x.min(lb.x)) + 1;
                        let dy = (p3.y.max(rb.y) - p3.y.min(rb.y)) + 1;
                        let area = dx * dy;
                        area
                    } else {
                        0
                    }
                }
                (1, -1) => {
                    println!("{:?} is (1,-1)", p3);
                    let lt = positions
                        .iter()
                        .find(|p| p.x == p3.x && p.y < p3.y)
                        .expect("Didn't find lt");
                    if let Some(rt) = positions.iter().find(|p| p.x < lt.x && p.y == lt.y) {
                        let dx = (p3.x.max(rt.x) - p3.x.min(rt.x)) + 1;
                        let dy = (p3.y.max(lt.y) - p3.y.min(lt.y)) + 1;
                        let area = dx * dy;
                        area
                    } else {
                        0
                    }
                }
                (-1, -1) => {
                    println!("{:?} is (-1,-1)", p3);
                    let rt = positions
                        .iter()
                        .find(|p| p.x == p3.x && p.y < p3.y)
                        .expect("Didn't find rt");
                    if let Some(lt) = positions.iter().find(|p| p.x < rt.x && p.y == rt.y) {
                        let dx = (p3.x.max(lt.x) - p3.x.min(lt.x)) + 1;
                        let dy = (p3.y.max(rt.y) - p3.y.min(rt.y)) + 1;
                        let area = dx * dy;
                        area
                    } else {
                        0
                    }
                }
                _ => panic!("This should not happen"),
            }
        })
        .max()
        .unwrap();

    max_area
}

///
/// (1,1)
/// #--
/// |
///
/// (-1, 1)
/// --#
///   |
///
/// (1, -1)
/// |
/// #--
///
/// (-1, -1)
///   |
/// --#
///
/// (1,1)
/// (1,-1)
/// (-1,1)
///
/// (1,1)
/// (-1,-1)
/// (1,1)
///

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = include_str!("../test.txt");
        assert_eq!(solve(input), 24);
    }
}
