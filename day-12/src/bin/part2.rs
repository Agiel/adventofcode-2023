use std::{
    collections::HashMap,
    sync::{Mutex, OnceLock},
};

use aocd::*;

#[aocd(2023, 12)]
fn main() {
    let input = input!();
    let result = part2(&input);
    dbg!(result);
}

struct Record {
    conditions: String,
    sequences: Vec<u32>,
}

fn parse(input: &str) -> Vec<Record> {
    input
        .lines()
        .map(|line| {
            let (conditions, sequences) = line.split_once(' ').unwrap();
            let conditions = vec![conditions; 5].join("?");
            let sequences = sequences
                .split(',')
                .map(|c| c.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
                .repeat(5);
            Record {
                conditions,
                sequences,
            }
        })
        .collect()
}

fn cache() -> &'static Mutex<HashMap<String, u64>> {
    static CACHE: OnceLock<Mutex<HashMap<String, u64>>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn cached_count_valid(conditions: &str, sequences: &[u32], num_broken: u32) -> u64 {
    let hash = format!(
        "{} {} {}",
        conditions,
        sequences
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join(","),
        num_broken
    );
    if let Some(cached) = cache().lock().unwrap().get(&hash) {
        return *cached;
    }
    let res = count_valid(conditions, sequences, num_broken);
    cache().lock().unwrap().insert(hash, res);
    res
}

fn count_valid(conditions: &str, sequences: &[u32], num_broken: u32) -> u64 {
    let mut sequences = sequences;
    let mut num_broken = num_broken;
    let mut pick_broken = true;
    if sequences.len() > 0 && num_broken == sequences[0] {
        if conditions.len() > 0 {
            match conditions.chars().next().unwrap() {
                '#' => return 0,
                '?' => pick_broken = false,
                _ => (),
            }
        }
        sequences = &sequences[1..];
        num_broken = 0;
    }

    if conditions.len() == 0 {
        if sequences.len() == 0 && num_broken == 0 {
            return 1;
        }
        return 0;
    }

    match conditions.chars().next().unwrap() {
        '.' => {
            if num_broken > 0 {
                0
            } else {
                cached_count_valid(&conditions[1..], sequences, num_broken)
            }
        }
        '#' => cached_count_valid(&conditions[1..], sequences, num_broken + 1),
        '?' => {
            let operational = if num_broken > 0 {
                0
            } else {
                cached_count_valid(&conditions[1..], sequences, num_broken)
            };
            let damaged = if pick_broken {
                cached_count_valid(&conditions[1..], sequences, num_broken + 1)
            } else {
                0
            };
            operational + damaged
        }
        _ => panic!(),
    }
}

fn part2(input: &str) -> u64 {
    let records = parse(&input);
    records
        .iter()
        .enumerate()
        .map(|(n, record)| {
            cache().lock().unwrap().clear();
            let count = count_valid(&record.conditions, &record.sequences[..], 0);
            println!("{}/{}", n + 1, records.len());
            count
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../../example1.txt");
        assert_eq!(part2(&input), 525152);
    }
}
