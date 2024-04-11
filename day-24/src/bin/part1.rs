use aocd::*;

#[aocd(2023, 24)]
fn main() {
    let input = input!();
    let result = part1(&input, 200_000_000_000_000., 400_000_000_000_000.);
    dbg!(result);
}

struct HailStone {
    pos: (f64, f64, f64),
    vel: (f64, f64, f64),
}

fn parse(input: &str) -> Option<Vec<HailStone>> {
    input
        .lines()
        .map(|line| {
            let (pos, vel) = line.split_once(" @ ")?;
            let mut pos = pos.split(", ").filter_map(|n| n.trim().parse::<f64>().ok());
            let pos = (pos.next()?, pos.next()?, pos.next()?);
            let mut vel = vel.split(", ").filter_map(|n| n.trim().parse::<f64>().ok());
            let vel = (vel.next()?, vel.next()?, vel.next()?);
            Some(HailStone { pos, vel })
        })
        .collect()
}

fn part1(input: &str, min: f64, max: f64) -> usize {
    let hailstones = parse(&input).unwrap();
    hailstones
        .iter()
        .enumerate()
        .map(|(n, hailstone)| {
            hailstones
                .iter()
                .skip(n)
                .filter_map(|other| {
                    let slope0 = hailstone.vel.1 / hailstone.vel.0;
                    let slope1 = other.vel.1 / other.vel.0;

                    // Parallell
                    if slope0 == slope1 {
                        return None;
                    }

                    // y = slope0 * (x - x0) + y0
                    // y = slope0 * x - slope0 * x0 + y0
                    // slope0 * x - slope0 * x0 + y0 = slope1 * x - slope1 * x1 + y1
                    // slope0 * x - slope 1 * x = slope0 * x0 - slope1 + x1 + y1 - y0
                    // x = (slope0 * x0 - slope1 * x1 + y1 - y0) / (slope0 - slope1)

                    let intersect_x =
                        (slope0 * hailstone.pos.0 - hailstone.pos.1 - slope1 * other.pos.0
                            + other.pos.1)
                            / (slope0 - slope1);
                    let intersect_y = slope0 * (intersect_x - hailstone.pos.0) + hailstone.pos.1;

                    // Make sure it's forward in time
                    if (intersect_x - hailstone.pos.0) / hailstone.vel.0 < 0.
                        || (intersect_x - other.pos.0) / other.vel.0 < 0.
                    {
                        return None;
                    }

                    (intersect_x >= min
                        && intersect_x <= max
                        && intersect_y >= min
                        && intersect_y <= max)
                        .then_some(())
                })
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../../example1.txt");
        assert_eq!(part1(&input, 7., 27.), 2);
    }
}
