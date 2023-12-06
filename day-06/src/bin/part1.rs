use aocd::*;

#[aocd(2023, 6)]
fn main() {
    let input = input!();
    let result = part1(&input);
    dbg!(result);
}

#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32,
}

fn parse(input: &str) -> Vec<Race> {
    let (times, distances) = input.split_once('\n').unwrap();
    times
        .split_whitespace()
        .skip(1)
        .zip(distances.split_whitespace().skip(1))
        .map(|(time, distance)| Race {
            time: time.parse().unwrap(),
            distance: distance.parse().unwrap(),
        })
        .collect::<Vec<_>>()
}

fn part1(input: &str) -> u32 {
    let races = parse(input);
    races
        .iter()
        .map(|race| {
            let mut records = 0;
            for hold_time in 1..race.time {
                let distance = hold_time * (race.time - hold_time);
                if distance > race.distance {
                    records += 1;
                }
            }
            records
        })
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../../example.txt");
        assert_eq!(part1(input), 288);
    }
}
