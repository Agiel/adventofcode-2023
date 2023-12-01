use regex::Regex;

fn main() {
    let input = include_str!("./input1.txt");
    let sum = part2(input);
    dbg!(sum);
}

fn part2(input: &str) -> u32 {
    let re_first = Regex::new(r"([1-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let re_last = Regex::new(r"([1-9]|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").unwrap();

    input
        .lines()
        .map(|line| {
            let first = to_num(re_first.find(line).unwrap().into());
            let reversed: String = line.chars().rev().collect();
            let last: &str = re_last.find(&reversed).unwrap().into();
            let last: String = last.chars().rev().collect();
            let last = to_num(&last);
            let num = format!("{}{}", first, last);
            num
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
