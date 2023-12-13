use aocd::*;

#[aocd(2023, 13)]
fn main() {
    let input = input!();
    let result = part2(&input);
    dbg!(result);
}

struct Pattern {
    pattern: Vec<String>,
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
        pattern: pattern.iter().map(|s| s.to_string()).collect(),
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

fn compare(a: &u32, b: &u32, with_smudge: bool) -> (bool, bool) {
    if a == b {
        (true, false)
    } else if with_smudge {
        if (a ^ b) as i32 & ((a ^ b) as i32 - 1) == 0 {
            dbg!(a, b, a.abs_diff(*b));
            (true, true)
        } else {
            (false, false)
        }
    } else {
        (false, false)
    }
}

fn find_reflection(pattern: &Vec<u32>) -> usize {
    let mut visited = Vec::new();
    pattern
        .iter()
        .enumerate()
        .find_map(|(n, row)| {
            let mut used_smudge = false;
            let eq = {
                if let Some(v) = visited.last() {
                    let (eq, used) = compare(v, row, !used_smudge);
                    used_smudge |= used;
                    eq
                } else {
                    false
                }
            };
            if eq {
                // dbg!(used_smudge, n);
                if pattern.iter().skip(n + 1).enumerate().all(|(i, next)| {
                    let reflection = visited.len() as i32 - i as i32 - 2;
                    if reflection >= 0 {
                        if let Some(v) = visited.get(reflection as usize) {
                            let (eq, used) = compare(v, next, !used_smudge);
                            used_smudge |= used;
                            eq
                        } else {
                            true
                        }
                    } else {
                        true
                    }
                }) {
                    if used_smudge {
                        return Some(n);
                    }
                }
            }
            visited.push(*row);
            None
        })
        .unwrap_or(0)
}

fn print_reflection(pattern: &Pattern, row: usize, column: usize) {
    pattern.pattern.iter().enumerate().for_each(|(r, line)| {
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
            if vertical == 0 {
                100 * horizontal
            } else {
                vertical
            }
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
