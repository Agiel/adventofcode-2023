use std::collections::{HashMap, HashSet};

use regex::Regex;

fn main() {
    let input = include_str!("./input1.txt");
    let sum = part2(input);
    let sum_alt = part2_alt(input);
    dbg!(sum);
    dbg!(sum_alt);
}

fn part2(input: &str) -> u32 {
    let mut cards = HashMap::new();
    let re = Regex::new(r"(\d+|\|)").unwrap();
    input
        .lines()
        .map(|line| {
            let mut winning = HashSet::new();
            let mut nums = re.find_iter(line);
            let game = nums.next().unwrap().as_str().parse::<u32>().unwrap();
            while let Some(num) = nums.next() {
                match num.as_str() {
                    "|" => break,
                    _ => {
                        winning.insert(num.as_str());
                    }
                }
            }

            let copies = cards.get(&game).unwrap_or(&0) + 1;
            let mut offset = 1;
            while let Some(num) = nums.next() {
                if !winning.contains(num.as_str()) {
                    continue;
                }
                if let Some(copies_next) = cards.get_mut(&(game + offset)) {
                    *copies_next += copies;
                } else {
                    cards.insert(game + offset, copies);
                }
                offset += 1;
            }

            copies
        })
        .sum()
}

fn part2_alt(input: &str) -> u32 {
    let mut cards = HashMap::new();
    input
        .lines()
        .enumerate()
        .map(|(game, line)| {
            let (_game, nums) = line.split_once(":").unwrap();

            let (winning, ours) = nums.split_once("|").unwrap();
            let winning = winning.split_whitespace().collect::<HashSet<_>>();
            let ours = ours.split_whitespace().collect::<HashSet<_>>();

            let score = winning.intersection(&ours).count();
            let copies = cards.get(&game).unwrap_or(&0) + 1;
            for i in 1..=score {
                if let Some(copies_next) = cards.get_mut(&(game + i)) {
                    *copies_next += copies;
                } else {
                    cards.insert(game + i, copies);
                }
            }
            copies
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(part2(input), 30);
        assert_eq!(part2(input), part2_alt(input));
    }
}
