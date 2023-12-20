use aocd::*;

#[aocd(2023, 18)]
fn main() {
    let input = input!();
    let result = part1(&input);
    dbg!(result);
}

struct Instruction<'a> {
    direction: &'a str,
    meters: i32,
    color: &'a str,
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let direction = parts.next().unwrap();
            let meters = parts.next().unwrap().parse::<i32>().unwrap();
            let color = parts.next().unwrap();
            Instruction {
                direction,
                meters,
                color,
            }
        })
        .collect()
}

fn get_points(instructions: &Vec<Instruction>) -> Vec<(i32, i32)> {
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

fn part1(input: &str) -> i32 {
    let instructions = parse(input);
    let points = get_points(&instructions);

    let mut area: i32 = points
        .iter()
        .zip(points.iter().skip(1))
        .map(|(a, b)| (a.1 + b.1) * (a.0 - b.0))
        .sum();
    let first = points.first().unwrap();
    let last = points.last().unwrap();
    area += (last.1 + first.1) * (last.0 - first.0);
    (area as f32 / 2.0) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../../example1.txt");
        assert_eq!(part1(&input), 62);
    }
}
