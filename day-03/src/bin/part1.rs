use aocd::*;
use regex::Regex;

#[aocd(2023, 3)]
fn main() {
    let input = input!();
    let sum = part1(&input);
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

    let re_sym = Regex::new(r"[^\d\w\s.]+").unwrap();
    input.lines().enumerate().for_each(|(row, line)| {
        re_sym.find_iter(line).for_each(|m| {
            parts.iter_mut().for_each(|part| {
                if (row as i32) >= part.row - 1
                    && (row as i32) <= part.row + 1
                    && (m.start() as i32) >= part.start - 1
                    && (m.end() as i32 - 1) <= part.end + 1
                {
                    part.valid = true;
                }
            })
        })
    });

    parts
        .iter()
        .filter_map(|part| {
            if part.valid {
                Some(part.value.parse::<u32>().unwrap())
            } else {
                None
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
        assert_eq!(part1(input), 4361);
    }
}
