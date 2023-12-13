use aocd::*;

#[aocd(2023, 13)]
fn main() {
    let input = input!();
    let result = part1(&input);
    dbg!(result);
}

struct Pattern {
    horizontal: Vec<u32>,
    vertical: Vec<u32>,
}

fn parse_pattern(pattern: &Vec<&str>) -> Pattern {
    let horizontal = pattern
        .iter()
        .map(|p: &&str| {
            p.chars().fold(0, |acc, c| {
                let mut acc: u32 = acc << 1;
                if c == '#' {
                    acc += 1;
                }
                acc
            })
        })
        .collect::<Vec<_>>();
    let vertical = (0..pattern[0].len())
        .map(|row| {
            pattern.iter().fold(0, |acc, line| {
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
    }
}

fn parse(input: &str) -> Vec<Pattern> {
    let mut patterns = Vec::new();
    let mut pattern = Vec::new();
    input.lines().for_each(|line| {
        if line.is_empty() {
            patterns.push(parse_pattern(&pattern));
            pattern = Vec::new();
        } else {
            pattern.push(line);
        }
    });
    patterns.push(parse_pattern(&pattern));
    patterns
}

fn find_reflection(pattern: &Vec<u32>) -> usize {
    let mut visited = Vec::new();
    pattern
        .iter()
        .enumerate()
        .find_map(|(n, row)| {
            if Some(row) == visited.last() {
                if pattern.iter().skip(n).enumerate().all(|(i, next)| {
                    let reflection = visited.len() as i32 - i as i32 - 1;
                    if reflection >= 0 {
                        if let Some(v) = visited.get(reflection as usize) {
                            v == next
                        } else {
                            true
                        }
                    } else {
                        true
                    }
                }) {
                    return Some(n);
                }
            }
            visited.push(*row);
            None
        })
        .unwrap_or(0)
}

fn part1(input: &str) -> usize {
    let patterns = parse(input);
    patterns
        .iter()
        .map(|pattern| {
            find_reflection(&pattern.vertical) + 100 * find_reflection(&pattern.horizontal)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../../example1.txt");
        assert_eq!(part1(&input), 405);
    }
}
