use aocd::*;

#[aocd(2023, 2)]
fn main() {
    let input = input!();
    let sum = part1(&input, 12, 13, 14);
    dbg!(sum);
}

fn part1(input: &str, red: u32, green: u32, blue: u32) -> u32 {
    input
        .lines()
        .filter_map(|line| {
            let mut split = line.split(":");
            let game = split.next().unwrap();
            let sets = split.next().unwrap();
            let id = game.split(" ").last().unwrap();

            let (mut r, mut g, mut b) = (0, 0, 0);
            sets.split(";").for_each(|set| {
                set.split(",").for_each(|color| {
                    let mut color_info = color.trim().split(" ");
                    let n: u32 = color_info.next().unwrap().parse().unwrap();
                    let c = color_info.next().unwrap();
                    match c {
                        "red" => r = std::cmp::max(r, n),
                        "green" => g = std::cmp::max(g, n),
                        "blue" => b = std::cmp::max(b, n),
                        _ => panic!(),
                    }
                })
            });
            if r <= red && g <= green && b <= blue {
                Some(id.parse::<u32>().unwrap())
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
        assert_eq!(part1(input, 12, 13, 14), 8);
    }
}
