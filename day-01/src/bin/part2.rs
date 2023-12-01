use regex::Regex;

fn main() {
    let input = include_str!("./input1.txt");
    let sum = part2(input);
    dbg!(sum);
}

fn part2(input: &str) -> u32 {
    let lines = input.split('\n').filter(|l| l.len() > 0).map(|line| {
        let re = Regex::new(r"([1-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
        let matches: Vec<&str> = re.find_iter(line).map(|m| m.as_str()).collect();
        let first = to_num(matches[0]);
        let last = to_num(matches[matches.len() - 1]);
        let num = format!("{}{}", first, last);
        dbg!(line, matches, &num);
        num
    });
    lines.map(|line| line.parse::<u32>().unwrap()).sum()
}

fn to_num(string: &str) -> &str {
    let first = string.as_bytes()[0];
    if first.is_ascii_digit() {
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
            _ => panic!(),
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
    }
}
