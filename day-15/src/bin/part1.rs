use aocd::*;

#[aocd(2023, 15)]
fn main() {
    let input = input!();
    let result = part1(&input);
    dbg!(result);
}

fn hash(string: &str) -> u32 {
    string
        .chars()
        .fold(0, |acc, c| ((acc + c as u32) * 17) % 256)
}

fn part1(input: &str) -> u32 {
    input.trim().split(',').map(|cmd| hash(cmd)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../../example1.txt");
        assert_eq!(part1(&input), 1320);
    }
}
