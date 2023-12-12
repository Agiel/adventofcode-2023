use std::{
    collections::HashMap,
    sync::{Mutex, OnceLock},
    time::Instant,
};

use aocd::*;

#[aocd(2023, 12)]
fn main() {
    let input = input!();
    let start_time = Instant::now();
    let result = part2(&input);
    println!("Elapsed time: {:?}", start_time.elapsed());
    dbg!(result);
}

struct Record {
    conditions: String,
    sequences: Vec<usize>,
}

fn parse(input: &str) -> Vec<Record> {
    input
        .lines()
        .map(|line| {
            let (conditions, sequences) = line.split_once(' ').unwrap();
            let conditions = vec![conditions; 5].join("?");
            let sequences = sequences
                .split(',')
                .map(|c| c.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
                .repeat(5);
            Record {
                conditions,
                sequences,
            }
        })
        .collect()
}

fn cache() -> &'static Mutex<HashMap<(usize, usize), u64>> {
    static CACHE: OnceLock<Mutex<HashMap<(usize, usize), u64>>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn cached_count_valid(conditions: &str, sequences: &[usize]) -> u64 {
    let hash = (conditions.len(), sequences.len());
    if let Some(cached) = cache().lock().unwrap().get(&hash) {
        return *cached;
    }
    let res = count_valid(conditions, sequences);
    cache().lock().unwrap().insert(hash, res);
    res
}

fn count_valid(conditions: &str, sequences: &[usize]) -> u64 {
    if conditions.len() == 0 {
        if sequences.len() == 0 {
            return 1;
        }
        return 0;
    }
    if sequences.len() == 0 {
        if conditions.contains('#') {
            return 0;
        }
        return 1;
    }

    let mut count = 0;
    let next_char = conditions.chars().next().unwrap();
    if ".?".contains(next_char) {
        count += cached_count_valid(&conditions[1..], sequences);
    }
    if "#?".contains(next_char) {
        if sequences[0] <= conditions.len()
            && !conditions[..sequences[0]].contains('.')
            && conditions.chars().nth(sequences[0]) != Some('#')
        {
            count += cached_count_valid(
                conditions.get(sequences[0] + 1..).unwrap_or(""),
                &sequences[1..],
            );
        }
    }
    count
}

fn part2(input: &str) -> u64 {
    let records = parse(&input);
    records
        .iter()
        .map(|record| {
            cache().lock().unwrap().clear();
            count_valid(&record.conditions, &record.sequences[..])
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
