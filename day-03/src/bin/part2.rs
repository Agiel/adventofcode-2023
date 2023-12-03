use regex::Regex;

fn main() {
    let input = include_str!("./input1.txt");
    let sum = part1(input);
    dbg!(sum);
}

#[derive(Debug)]
struct Part<'a> {
    row: i32,
    start: i32,
    end: i32,
    value: &'a str,
    valid: bool,
}

fn part1(input: &str) -> u32 {
    let re_nr = Regex::new(r"\d+").unwrap();
    let mut parts: Vec<Part> = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            re_nr
                .find_iter(line)
                .map(|m| Part {
                    row: row as i32,
                    start: m.start() as i32,
                    end: m.end() as i32 - 1,
                    value: m.as_str(),
                    valid: false,
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let re_sym = Regex::new(r"\*+").unwrap();
    input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            re_sym
                .find_iter(line)
                .filter_map(|m| {
                    let mut adjacent: u32 = 0;
                    let mut ratio: u32 = 1;
                    parts.iter_mut().for_each(|part| {
                        if (row as i32) >= part.row - 1
                            && (row as i32) <= part.row + 1
                            && (m.start() as i32) >= part.start - 1
                            && (m.end() as i32 - 1) <= part.end + 1
                        {
                            adjacent += 1;
                            ratio *= part.value.parse::<u32>().unwrap();
                        }
                    });
                    if adjacent == 2 {
                        Some(ratio)
                    } else {
                        None
                    }
                })
                .sum::<u32>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(part1(input), 467835);
    }
}
