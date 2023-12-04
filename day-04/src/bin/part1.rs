use std::collections::HashSet;

use regex::Regex;

fn main() {
    let input = include_str!("./input1.txt");
    let sum = part1(input);
    dbg!(sum);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(part1(input), 13);
    }
}
