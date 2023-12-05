use aocd::*;

#[aocd(2023, 5)]
fn main() {
    let result = part2_alt(&input!());
    dbg!(result);
}

#[derive(Debug)]
struct Map {
    dst_start: u64,
    src_start: u64,
    length: u64,
}

struct Parsed {
    seeds: Vec<(u64, u64)>,
    maps: Vec<Vec<Map>>,
}

fn parse(input: &str) -> Parsed {
    let mut lines = input.lines();
    let (_, seeds) = lines.next().unwrap().split_once(":").unwrap();
    let seeds = seeds
        .split_whitespace()
        .map(|seed| seed.parse::<u64>().unwrap())
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[1]))
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

    Parsed { seeds, maps }
}

fn part2(input: &str) -> u64 {
    let parsed = parse(input);

    parsed
        .seeds
        .iter()
        .map(|seed| {
            let length = seed.1;
            let seed = seed.0;
            (seed..=seed + length)
                .map(|s| {
                    let mut location = s.clone();
                    parsed.maps.iter().for_each(|inner| {
                        for map in inner.iter() {
                            if location >= map.src_start && location <= map.src_start + map.length {
                                location = map.dst_start + location - map.src_start;
                                break;
                            }
                        }
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
        for inner in parsed.maps.iter().rev() {
            for map in inner.iter() {
                if seed >= map.dst_start && seed <= map.dst_start + map.length {
                    seed = map.src_start + seed - map.dst_start;
                    break;
                }
            }
        }
        for seed_range in parsed.seeds.iter() {
            if seed >= seed_range.0 && seed <= seed_range.0 + seed_range.1 {
                return location;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../../example.txt");
        assert_eq!(part2(input), 46);
    }
}
