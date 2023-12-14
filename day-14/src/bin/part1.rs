use aocd::*;

#[aocd(2023, 14)]
fn main() {
    let input = input!();
    let result = part1(&input);
    dbg!(result);
}

fn part1(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<_>>();
    let num_rows = lines.len();
    (0..lines[0].len())
        .map(|c| {
            let mut last = 0;
            lines
                .iter()
                .enumerate()
                .fold(0usize, |acc, (r, line)| match line.chars().nth(c) {
                    Some('O') => {
                        last += 1;
                        acc + (num_rows - last + 1)
                    }
                    Some('#') => {
                        last = r + 1;
                        acc
                    }
                    _ => acc,
                })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../../example1.txt");
        assert_eq!(part1(&input), 136);
    }
}
