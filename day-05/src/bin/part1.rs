use aocd::*;

#[aocd(2023, 5)]
fn main() {
    let result = part1(&input!());
    dbg!(result);
}

#[derive(Debug)]
struct Map {
    dst_start: u64,
    src_start: u64,
    length: u64,
}

fn part1(input: &str) -> u64 {
    let mut lines = input.lines();
    let (_, seeds) = lines.next().unwrap().split_once(":").unwrap();
    let seeds = seeds
        .split_whitespace()
        .map(|seed| seed.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    // Skip to first map
    lines.next();
    lines.next();

    let mut maps: Vec<Vec<Map>> = Vec::new();
    maps.push(Vec::new());
    let mut i = 0;
    while let Some(line) = lines.next() {
        if line.chars().next().is_some() {
            let m: Vec<_> = line
                .split_whitespace()
                .map(|d| d.parse::<u64>().unwrap())
                .collect();
            if i >= maps.len() {
                maps.push(Vec::new());
            }
            maps[i].push(Map {
                dst_start: m[0],
                src_start: m[1],
                length: m[2],
            });
        } else {
            lines.next();
            i += 1;
        }
    }

    seeds.iter().fold(u64::MAX, |acc, seed| {
        let mut location = seed.clone();
        maps.iter().for_each(|inner| {
            for map in inner.iter() {
                if location >= map.src_start && location <= map.src_start + map.length {
                    location = map.dst_start + location - map.src_start;
                    break;
                }
            }
        });
        location.min(acc)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../../example.txt");
        assert_eq!(part1(input), 35);
    }
}
