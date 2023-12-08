use std::collections::BTreeMap;

use aocd::*;

#[aocd(2023, 8)]
fn main() {
    let input = input!();
    let result = part1(&input);
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

fn part1(input: &str) -> u32 {
    let map = parse(input);
    let mut steps = 0;
    let mut pos = "AAA";
    loop {
        for c in map.directions.chars() {
            steps += 1;
            let node = map.nodes.get(pos).unwrap();
            pos = match c {
                'L' => node.0,
                'R' => node.1,
                _ => panic!(),
            };
            if pos == "ZZZ" {
                return steps;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../../example1.txt");
        assert_eq!(part1(&input), 2);
        let input = include_str!("../../example2.txt");
        assert_eq!(part1(&input), 6);
    }
}
