use std::{collections::BTreeMap, time::Instant};

use aocd::*;
use num::integer;

#[aocd(2023, 8)]
fn main() {
    let input = input!();
    let start_time = Instant::now();
    let result = part2(&input);
    println!("Elapsed time: {:?}", start_time.elapsed());
    dbg!(result);
}

#[derive(Debug)]
struct Map<'a> {
    directions: &'a str,
    nodes: BTreeMap<&'a str, (&'a str, &'a str)>,
}

fn parse(input: &str) -> Map {
    let mut lines = input.lines();
    let directions = lines.next().unwrap();
    let nodes = lines
        .skip(1) // Empty line
        .map(|line| {
            let (key, value) = line.split_once(" = ").unwrap();
            let value = value
                .split_once(", ")
                .map(|(l, r)| (l.strip_prefix("(").unwrap(), r.strip_suffix(")").unwrap()))
                .unwrap();
            (key, value)
        })
        .collect::<BTreeMap<_, _>>();

    Map { directions, nodes }
}

fn part2(input: &str) -> u64 {
    let map = parse(input);
    let positions = map
        .nodes
        .iter()
        .filter_map(|(key, _)| key.ends_with('A').then_some(*key))
        .collect::<Vec<_>>();

    let loops = positions
        .iter()
        .map(|pos| {
            let mut visited = BTreeMap::<(&str, usize), u64>::new();
            let mut pos = *pos;
            let mut steps = 0u64;
            loop {
                for (i, c) in map.directions.chars().enumerate() {
                    steps += 1;
                    let node = map.nodes.get(pos).unwrap();
                    pos = match c {
                        'L' => node.0,
                        'R' => node.1,
                        _ => panic!(),
                    };
                    if pos.ends_with('Z') {
                        if visited.contains_key(&(pos, i)) {
                            return visited;
                        } else {
                            visited.insert((pos, i), steps);
                        }
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    // The different start positions only seem to end up at one goal each. So
    // just extract how many steps it took to get there and calculate the
    // lowest common multiple.
    loops
        .iter()
        .flat_map(|l| l.iter().map(|(_, v)| v))
        .fold(1u64, |acc, steps| integer::lcm(acc, *steps))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../../example3.txt");
        assert_eq!(part2(&input), 6);
    }
}
