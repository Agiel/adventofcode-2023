use aocd::*;
use regex::Regex;
use std::collections::HashSet;

#[aocd(2023, 4)]
fn main() {
    let input = input!();
    let sum = part1(&input);
    let sum_alt = part1_alt(&input);

    dbg!(sum);
    dbg!(sum_alt);
}

fn part1(input: &str) -> u32 {
    let re = Regex::new(r"(\d+|\|)").unwrap();
    input
        .lines()
        .map(|line| {
            let mut winning = HashSet::new();
            let mut nums = re.find_iter(line);
            let _game = nums.next();
            while let Some(num) = nums.next() {
                match num.as_str() {
                    "|" => break,
                    _ => {
                        winning.insert(num.as_str());
                    }
                }
            }

            let mut score = 0;
            while let Some(num) = nums.next() {
                if !winning.contains(num.as_str()) {
                    continue;
                }
                score = if score == 0 { 1 } else { score * 2 }
            }
            score
        })
        .sum()
}

fn part1_alt(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (_game, nums) = line.split_once(":").unwrap();

            let (winning, ours) = nums.split_once("|").unwrap();
            let winning = winning.split_whitespace().collect::<HashSet<_>>();
            let ours = ours.split_whitespace().collect::<HashSet<_>>();

            let score = winning.intersection(&ours).count();
            if score > 0 {
                2u32.pow(score as u32 - 1)
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(part1(input), 13);
        assert_eq!(part1(input), part1_alt(input));
    }
}
