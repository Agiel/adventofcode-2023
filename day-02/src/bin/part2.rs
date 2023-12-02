fn main() {
    let input = include_str!("./input1.txt");
    let sum = part2(input);
    dbg!(sum);
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(":");
            let _game = split.next().unwrap();
            let sets = split.next().unwrap();

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

            r * g * b
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(part2(input), 2286);
    }
}
