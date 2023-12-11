use aocd::*;

#[aocd(2023, 11)]
fn main() {
    let input = input!();
    let result = part2(&input, 1000000);
    dbg!(result);
}

fn part2(input: &str, exp_factor: usize) -> usize {
    let mut galaxies = Vec::<_>::new();

    // Map galaxies while expanding vertically
    let mut expansion_y = 0;
    input.lines().enumerate().for_each(|(y, line)| {
        let mut empty = true;
        line.chars().enumerate().for_each(|(x, c)| {
            if c == '#' {
                galaxies.push((x, y + expansion_y));
                empty = false;
            }
        });
        if empty {
            expansion_y += exp_factor - 1;
        }
    });

    // Sort the galaxies by x coordinate
    galaxies.sort();

    // Expand horizontally
    let mut expansion_x = 0;
    let mut last_x = 0;
    galaxies.iter_mut().for_each(|coord| {
        if coord.0 - last_x > 1 {
            expansion_x += (coord.0 - last_x - 1) * (exp_factor - 1);
        }
        last_x = coord.0;
        coord.0 += expansion_x;
    });

    // Calculate distances
    galaxies
        .iter()
        .enumerate()
        .map(|(i, g1)| {
            galaxies
                .iter()
                .skip(i)
                .map(|g2| (g1.0.abs_diff(g2.0) + g1.1.abs_diff(g2.1)))
                .sum::<usize>()
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../../example1.txt");
        assert_eq!(part2(&input, 10), 1030);
        assert_eq!(part2(&input, 100), 8410);
    }
}
