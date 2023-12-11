use std::collections::BTreeMap;

use aocd::*;

#[aocd(2023, 11)]
fn main() {
    let input = input!();
    let result = part1(&input);
    dbg!(result);
}

fn part1(input: &str) -> usize {
    let mut galaxy_map = BTreeMap::<(usize, usize), usize>::new();
    let mut galaxies = Vec::<_>::new();

    // Map galaxies while expanding vertically
    let mut expansion_y = 0;
    input.lines().enumerate().for_each(|(y, line)| {
        let mut empty = true;
        line.chars().enumerate().for_each(|(x, c)| {
            if c == '#' {
                galaxy_map.insert((x, y), galaxies.len());
                galaxies.push((x, y + expansion_y));
                empty = false;
            }
        });
        if empty {
            expansion_y += 1;
        }
    });

    // Expand horizontally
    let mut expansion_x = 0;
    let mut last_x = 0;
    // The BTreeMap sorts the galaxies by x coordinate
    galaxy_map.iter().for_each(|(coord, galaxy)| {
        if coord.0 - last_x > 1 {
            expansion_x += coord.0 - last_x - 1;
        }
        last_x = coord.0;
        galaxies[*galaxy].0 += expansion_x;
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
        assert_eq!(part1(&input), 374);
    }
}
