use aocd::*;

#[aocd(2023, 12)]
fn main() {
    let input = input!();
    let result = part1(&input);
    dbg!(result);
}

struct Record<'a> {
    conditions: &'a str,
    sequences: Vec<usize>,
}

fn parse(input: &str) -> Vec<Record> {
    input
        .lines()
        .map(|line| {
            let (conditions, sequences) = line.split_once(' ').unwrap();
            let sequences = sequences
                .split(',')
                .map(|c| c.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            Record {
                conditions,
                sequences,
            }
        })
        .collect()
}

fn count_valid(conditions: &str, sequences: &[usize]) -> u32 {
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
        count += count_valid(&conditions[1..], sequences);
    }
    if "#?".contains(next_char) {
        if sequences[0] <= conditions.len()
            && !conditions[..sequences[0]].contains('.')
            && conditions.chars().nth(sequences[0]) != Some('#')
        {
            count += count_valid(
                conditions.get(sequences[0] + 1..).unwrap_or(""),
                &sequences[1..],
            );
        }
    }
    count
}

fn part1(input: &str) -> u32 {
    let records = parse(&input);
    records
        .iter()
        .map(|record| {
            let count = count_valid(record.conditions, &record.sequences[..]);
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
