use std::time::Instant;

use aocd::*;

#[aocd(2023, 6)]
fn main() {
    let input = input!();
    let start_time = Instant::now();
    let result = part2(&input);
    println!("Elapsed time: {:?}", start_time.elapsed());
    dbg!(result);
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

fn parse(input: &str) -> Race {
    let (times, distances) = input.split_once('\n').unwrap();
    let time = times
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    let distance = distances
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    Race { time, distance }
}

fn part2(input: &str) -> u64 {
    let race = parse(input);
    let mut records = 0;
    for hold_time in 1..race.time {
        let distance = hold_time * (race.time - hold_time);
        if distance > race.distance {
            records += 1;
        }
    }
    records
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../../example.txt");
        assert_eq!(part2(input), 71503);
    }
}
