use aho_corasick::AhoCorasick;
use aocd::*;
use regex::Regex;

#[aocd(2023, 1)]
fn main() {
    let input = input!();
    let sum = part2(&input);
    dbg!(sum);
    let sum_alt = part2_alt(&input);
    dbg!(sum_alt);
}

fn part2(input: &str) -> u32 {
    let re_first = Regex::new(r"([1-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let re_last = Regex::new(r".*([1-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();

    input
        .lines()
        .map(|line| {
            let first = to_num(re_first.find(line).unwrap().as_str());
            let last = to_num(re_last.captures(line).unwrap().get(1).unwrap().as_str());
            format!("{}{}", first, last)
        })
        .map(|line| line.parse::<u32>().unwrap())
        .sum()
}

fn part2_alt(input: &str) -> u32 {
    let patterns = &[
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];
    let ac = AhoCorasick::new(patterns).unwrap();

    input
        .lines()
        .map(|line| {
            let mut matches = ac.find_overlapping_iter(line);
            let first = matches.next().unwrap();
            let first = to_num(&line[first.start()..first.end()]);
            let last = if let Some(last) = matches.last() {
                to_num(&line[last.start()..last.end()])
            } else {
                first
            };
            format!("{}{}", first, last)
        })
        .map(|line| line.parse::<u32>().unwrap())
        .sum()
}

fn to_num(string: &str) -> &str {
    let first = string.chars().next().unwrap();
    if first.is_numeric() {
        string
    } else {
        match string {
            "one" => "1",
            "two" => "2",
            "three" => "3",
            "four" => "4",
            "five" => "5",
            "six" => "6",
            "seven" => "7",
            "eight" => "8",
            "nine" => "9",
            _ => panic!("{string}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example2.txt");
        assert_eq!(part2(input), 281);
        assert_eq!(part2(input), part2_alt(input));
    }
}
