use std::collections::{BTreeMap, BTreeSet, VecDeque};

use aocd::*;

#[aocd(2023, 22)]
fn main() {
    let input = input!();
    let result = part2(&input);
    dbg!(result);
}

#[derive(Debug)]
struct Brick {
    start: (u32, u32, u32),
    end: (u32, u32, u32),
}

#[derive(Default, Debug, Clone)]
struct Joint {
    supporting: BTreeSet<usize>,
    supported_by: BTreeSet<usize>,
}

fn parse(input: &str) -> Vec<Brick> {
    input
        .lines()
        .filter_map(|line| {
            let (start, end) = line.split_once('~')?;
            let (mut start, mut end) = (
                start.split(',').filter_map(|n| n.parse::<u32>().ok()),
                end.split(',').filter_map(|n| n.parse::<u32>().ok()),
            );
            Some(Brick {
                start: (start.next()?, start.next()?, start.next()?),
                end: (end.next()?, end.next()?, end.next()?),
            })
        })
        .collect()
}

#[derive(Default)]
struct Point {
    z: u32,
    index: usize,
}

fn settle(bricks: &mut Vec<Brick>) -> Vec<Joint> {
    bricks.sort_by_key(|b| b.start.2);
    let mut bottom = BTreeMap::<(u32, u32), Point>::new();
    let mut brick_joints = vec![Joint::default(); bricks.len()];
    bricks.iter_mut().enumerate().for_each(|(i, b)| {
        // Find highest point beneath brick
        let mut min = 0;
        for x in b.start.0..=b.end.0 {
            for y in b.start.1..=b.end.1 {
                min = min.max(bottom.get(&(x, y)).unwrap_or(&Point::default()).z);
            }
        }
        let height = b.end.2 - b.start.2;
        b.start.2 = min + 1;
        b.end.2 = min + 1 + height;

        // Update highest point
        for x in b.start.0..=b.end.0 {
            for y in b.start.1..=b.end.1 {
                bottom
                    .entry((x, y))
                    .and_modify(|p| {
                        if p.z == min {
                            brick_joints[p.index].supporting.insert(i);
                            brick_joints[i].supported_by.insert(p.index);
                        }
                        p.z = b.end.2;
                        p.index = i;
                    })
                    .or_insert(Point {
                        z: b.end.2,
                        index: i,
                    });
            }
        }
    });
    brick_joints
}

fn count_supporting(joints: &Vec<Joint>, index: usize) -> usize {
    let mut joints = joints.clone();
    let mut to_visit = VecDeque::from([index]);
    let mut seen = BTreeSet::from([index]);
    let mut count = 0;
    while let Some(current) = to_visit.pop_front() {
        for sup in joints[current].supporting.clone().iter() {
            let supported_by = &mut joints[*sup].supported_by;
            supported_by.remove(&current);

            if supported_by.len() == 0 && !seen.contains(sup) {
                seen.insert(*sup);
                to_visit.push_back(*sup);
                count += 1;
            }
        }
    }

    count
}

fn part2(input: &str) -> usize {
    let mut bricks = parse(input);
    let joints = settle(&mut bricks);
    joints
        .iter()
        .enumerate()
        .map(|(i, _)| count_supporting(&joints, i))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../../example1.txt");
        assert_eq!(part2(&input), 7);
    }
}
