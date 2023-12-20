use aocd::*;

#[aocd(2023, 18)]
fn main() {
    let input = input!();
    let result = part2(&input);
    dbg!(result);
}

struct Instruction<'a> {
    direction: &'a str,
    meters: i64,
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let parts = line.split_whitespace();
            let hex = parts.last().unwrap();
            // hex looks like (#abcdef) where the first five digits encode the distance.
            let meters = &hex[2..hex.len() - 2];
            let direction = &hex[hex.len() - 2..hex.len() - 1];

            let meters = i64::from_str_radix(meters, 16).unwrap();
            let direction = match direction {
                "0" => "R",
                "1" => "D",
                "2" => "L",
                "3" => "U",
                _ => panic!(),
            };

            Instruction { direction, meters }
        })
        .collect()
}

fn get_points(instructions: &Vec<Instruction>) -> Vec<(i64, i64)> {
    let mut points = Vec::new();
    points.push((0, 0));
    for (n, i) in instructions.iter().enumerate() {
        let last = points.last().unwrap();
        let next_dir = if n == instructions.len() - 1 {
            "R"
        } else {
            instructions[n + 1].direction
        };
        let last_dir = if n == 0 {
            "U"
        } else {
            instructions[n - 1].direction
        };
        match i.direction {
            "U" => {
                let mut offset = 0;
                if next_dir == "R" {
                    offset += 1;
                }
                if last_dir == "R" {
                    offset -= 1;
                }
                points.push((last.0, last.1 - i.meters - offset))
            }
            "D" => {
                let mut offset = 0;
                if next_dir == "L" {
                    offset += 1;
                }
                if last_dir == "L" {
                    offset -= 1;
                }
                points.push((last.0, last.1 + i.meters + offset))
            }
            "L" => {
                let mut offset = 0;
                if next_dir == "U" {
                    offset += 1;
                }
                if last_dir == "U" {
                    offset -= 1;
                }
                points.push((last.0 - i.meters - offset, last.1))
            }
            "R" => {
                let mut offset = 0;
                if next_dir == "D" {
                    offset += 1;
                }
                if last_dir == "D" {
                    offset -= 1;
                }
                points.push((last.0 + i.meters + offset, last.1))
            }
            _ => (),
        }
    }
    points
}

fn part2(input: &str) -> i64 {
    let instructions = parse(input);
    let points = get_points(&instructions);

    let mut area: i64 = points
        .iter()
        .zip(points.iter().skip(1))
        .map(|(a, b)| (a.1 + b.1) * (a.0 - b.0))
        .sum();
    let first = points.first().unwrap();
    let last = points.last().unwrap();
    area += (last.1 + first.1) * (last.0 - first.0);
    (area as f64 / 2.0) as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../../example1.txt");
        assert_eq!(part2(&input), 952408144115);
    }
}
