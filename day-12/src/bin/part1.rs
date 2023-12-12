use aocd::*;

#[aocd(2023, 12)]
fn main() {
    let input = input!();
    let result = part1(&input);
    dbg!(result);
}

struct Record<'a> {
    conditions: &'a str,
    sequences: Vec<u32>,
}

fn parse(input: &str) -> Vec<Record> {
    input
        .lines()
        .map(|line| {
            let (conditions, sequences) = line.split_once(' ').unwrap();
            let sequences = sequences
                .split(',')
                .map(|c| c.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            Record {
                conditions,
                sequences,
            }
        })
        .collect()
}

fn count_valid(conditions: &str, sequences: &[u32], num_broken: u32) -> u32 {
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
                count_valid(&conditions[1..], sequences, num_broken)
            }
        }
        '#' => count_valid(&conditions[1..], sequences, num_broken + 1),
        '?' => {
            let operational = if num_broken > 0 {
                0
            } else {
                count_valid(&conditions[1..], sequences, num_broken)
            };
            let damaged = if pick_broken {
                count_valid(&conditions[1..], sequences, num_broken + 1)
            } else {
                0
            };
            operational + damaged
        }
        _ => panic!(),
    }
}

fn part1(input: &str) -> u32 {
    let records = parse(&input);
    records
        .iter()
        .map(|record| {
            let count = count_valid(record.conditions, &record.sequences[..], 0);
            println!(
                "{:25}{:16}: {}",
                record.conditions,
                record
                    .sequences
                    .iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
                count
            );
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
        assert_eq!(part1(&input), 21);
    }
}
