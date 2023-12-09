use aocd::*;

#[aocd(2023, 9)]
fn main() {
    let input = input!();
    let result = part1(&input);
    dbg!(result);
}

fn extrapolate(history: &Vec<i64>) -> i64 {
    let diffs = history
        .iter()
        .zip(history.iter().skip(1))
        .map(|(a, b)| b - a)
        .collect::<Vec<_>>();
    if diffs.iter().all(|d| *d == 0) {
        *history.last().unwrap()
    } else {
        history.last().unwrap() + extrapolate(&diffs)
    }
}

fn part1(input: &str) -> i64 {
    let histories = input
        .lines()
        .map(|line| line.split_whitespace().map(|s| s.parse::<i64>().unwrap()));

    histories
        .map(|history| extrapolate(&history.collect::<Vec<_>>()))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../../example.txt");
        assert_eq!(part1(&input), 114);
    }
}
