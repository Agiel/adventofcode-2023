use aocd::*;

#[aocd(2023, 1)]
fn main() {
    let input = input!();
    let sum = part1(&input);
    dbg!(sum);
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            line.chars()
                .filter(|c| c.is_numeric())
                .collect::<Vec<char>>()
        })
        .map(|line| format!("{}{}", line[0], line[line.len() - 1]))
        .map(|line| line.parse::<u32>().unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(part1(input), 142);
    }
}
