use std::time::Instant;

use aocd::*;

#[aocd(2023, 5)]
fn main() {
    let input = input!();
    // WARNING!: The bad solution takes ~3 minutes to run
    // let start_time = Instant::now();
    // let result = part2(&input);
    // dbg!(result);
    // println!("Bad brute force, elapsed time: {:?}", start_time.elapsed());
    let start_time = Instant::now();
    let result_alt = part2_alt(&input);
    dbg!(result_alt);
    println!(
        "Better brute force, elapsed time: {:?}",
        start_time.elapsed()
    );
    let start_time = Instant::now();
    let result_alt2 = part2_alt2(&input);
    dbg!(result_alt2);
    println!("Good solution, elapsed time: {:?}", start_time.elapsed());
}

#[derive(Debug)]
struct Map {
    dst_start: u64,
    src_start: u64,
    length: u64,
}

struct Mapper {
    maps: Vec<Map>,
}

impl Mapper {
    fn new() -> Self {
        Self { maps: Vec::new() }
    }

    fn map_one(&self, seed: u64) -> u64 {
        for map in self.maps.iter() {
            if seed >= map.src_start && seed <= map.src_start + map.length {
                return map.dst_start + seed - map.src_start;
            }
        }
        seed
    }

    fn map_one_rev(&self, location: u64) -> u64 {
        for map in self.maps.iter() {
            if location >= map.dst_start && location <= map.dst_start + map.length {
                return map.src_start + location - map.dst_start;
            }
        }
        location
    }

    fn map_range(&self, seed_ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
        let mut seed_ranges = seed_ranges;
        let mut mapped = Vec::new();
        for map in self.maps.iter() {
            let mut unmapped = Vec::new();
            for range in seed_ranges.iter() {
                if range.1 >= map.src_start && range.0 <= map.src_start + map.length {
                    if range.0 < map.src_start {
                        unmapped.push((range.0, map.src_start - range.0));
                    }
                    mapped.push((
                        range.0.max(map.src_start) + map.dst_start - map.src_start,
                        range.1.min(map.src_start + map.length) + map.dst_start - map.src_start,
                    ));
                    if range.1 > map.src_start + map.length {
                        unmapped.push((map.src_start + map.length, range.1));
                    }
                } else {
                    unmapped.push(*range);
                }
            }
            seed_ranges = unmapped;
        }
        mapped.append(&mut seed_ranges);
        mapped
    }
}

struct Parsed {
    seeds: Vec<(u64, u64)>,
    mappers: Vec<Mapper>,
}

fn parse(input: &str) -> Parsed {
    let mut lines = input.lines();
    let (_, seeds) = lines.next().unwrap().split_once(":").unwrap();
    let seeds = seeds
        .split_whitespace()
        .map(|seed| seed.parse::<u64>().unwrap())
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[0] + chunk[1]))
        .collect::<Vec<_>>();

    // Skip to first map
    lines.next();
    lines.next();

    let mut mappers: Vec<Mapper> = Vec::new();
    mappers.push(Mapper::new());
    let mut i = 0;
    while let Some(line) = lines.next() {
        if line.chars().next().is_some() {
            let m: Vec<_> = line
                .split_whitespace()
                .map(|d| d.parse::<u64>().unwrap())
                .collect();
            if i >= mappers.len() {
                mappers.push(Mapper::new());
            }
            mappers[i].maps.push(Map {
                dst_start: m[0],
                src_start: m[1],
                length: m[2],
            });
        } else {
            lines.next();
            i += 1;
        }
    }

    Parsed { seeds, mappers }
}

fn part2(input: &str) -> u64 {
    let parsed = parse(input);

    parsed
        .seeds
        .iter()
        .map(|seed| {
            (seed.0..seed.1)
                .map(|s| {
                    let mut location = s.clone();
                    parsed.mappers.iter().for_each(|mapper| {
                        location = mapper.map_one(location);
                    });
                    location
                })
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}

fn part2_alt(input: &str) -> u64 {
    let parsed = parse(input);

    let mut location = 0;
    loop {
        location += 1;
        let mut seed = location.clone();
        for mapper in parsed.mappers.iter().rev() {
            seed = mapper.map_one_rev(seed);
        }
        for seed_range in parsed.seeds.iter() {
            if seed >= seed_range.0 && seed <= seed_range.1 {
                return location;
            }
        }
    }
}

fn part2_alt2(input: &str) -> u64 {
    let parsed = parse(input);

    parsed
        .seeds
        .iter()
        .map(|seed| {
            parsed
                .mappers
                .iter()
                .fold(vec![seed.clone()], |acc, mapper| mapper.map_range(acc))
                .iter()
                .map(|location| location.0)
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../../example.txt");
        assert_eq!(part2_alt2(input), 46);
    }
}
