use std::time::Instant;

use aocd::*;

#[aocd(2023, 13)]
fn main() {
    let input = input!();
    let start_time = Instant::now();
    let result = part2(&input);
    println!("Elapsed time: {:?}", start_time.elapsed());
    dbg!(result);
}

struct Pattern<'a> {
    pattern: &'a str,
    horizontal: Vec<u32>,
    vertical: Vec<u32>,
}

fn parse_pattern(pattern: &str) -> Pattern {
    let horizontal = pattern
        .lines()
        .map(|p| {
            p.chars().fold(0, |acc, c| {
                let mut acc: u32 = acc << 1;
                if c == '#' {
                    acc += 1;
                }
                acc
            })
        })
        .collect::<Vec<_>>();
    let line_len = pattern.find('\n').unwrap();
    let vertical = (0..line_len)
        .map(|row| {
            pattern.lines().fold(0, |acc, line| {
                let mut acc: u32 = acc << 1;
                if line.chars().nth(row) == Some('#') {
                    acc += 1;
                }
                acc
            })
        })
        .collect::<Vec<_>>();
    Pattern {
        horizontal,
        vertical,
        pattern,
    }
}

fn parse(input: &str) -> Vec<Pattern> {
    input
        .split("\n\n")
        .map(|pattern| parse_pattern(pattern))
        .collect()
}

fn compare(a: &u32, b: &u32, with_smudge: bool) -> (bool, bool) {
    if a == b {
        (true, false)
    } else if with_smudge {
        if (a ^ b) as i32 & ((a ^ b) as i32 - 1) == 0 {
            (true, true)
        } else {
            (false, false)
        }
    } else {
        (false, false)
    }
}

fn find_reflection(pattern: &Vec<u32>) -> usize {
    (1..pattern.len())
        .find_map(|n| {
            let mut used_smudge = false;
            (pattern
                .iter()
                .skip(n)
                .zip(pattern.iter().take(n).rev())
                .all(|(a, b)| {
                    let (eq, used) = compare(a, b, !used_smudge);
                    used_smudge |= used;
                    eq
                })
                && used_smudge)
                .then_some(n)
        })
        .unwrap_or(0)
}

fn print_reflection(pattern: &Pattern, row: usize, column: usize) {
    pattern.pattern.lines().enumerate().for_each(|(r, line)| {
        if row > 0 && r == row {
            println!("{}", "-".repeat(line.len()));
        }
        line.chars().enumerate().for_each(|(c, char)| {
            if c > 0 && c == column {
                print!("|");
            }
            print!("{}", char);
        });
        print!("\n");
    });
    print!("\n");
}

fn part2(input: &str) -> usize {
    let patterns = parse(input);
    patterns
        .iter()
        .map(|pattern| {
            let vertical = find_reflection(&pattern.vertical);
            let horizontal = find_reflection(&pattern.horizontal);
            print_reflection(&pattern, horizontal, vertical);
            vertical + 100 * horizontal
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../../example1.txt");
        assert_eq!(part2(&input), 400);
    }
}
