use std::collections::HashSet;

use aocd::*;

#[aocd(2023, 21)]
fn main() {
    let input = input!();
    let result = part1(&input, 64);
    dbg!(result);
}

fn parse(input: &str) -> ((i32, i32), Vec<Vec<bool>>) {
    let mut start = (0, 0);
    let map = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| match c {
                    'S' => {
                        start = (row as i32, col as i32);
                        true
                    }
                    '.' => true,
                    _ => false,
                })
                .collect()
        })
        .collect();
    (start, map)
}

fn get_neighbours(map: &Vec<Vec<bool>>, pos: (i32, i32)) -> Vec<(i32, i32)> {
    [(1, 0), (-1, 0), (0, 1), (0, -1)]
        .iter()
        .filter_map(|dir| {
            let neighbour = (pos.0 + dir.0, pos.1 + dir.1);
            if neighbour.0 >= 0 && neighbour.1 >= 0 {
                (Some(&true)
                    == map
                        .get(neighbour.0 as usize)
                        .and_then(|r| r.get(neighbour.1 as usize)))
                .then_some(neighbour)
            } else {
                None
            }
        })
        .collect()
}

fn evolve(cells: &HashSet<(i32, i32)>, map: &Vec<Vec<bool>>) -> HashSet<(i32, i32)> {
    cells.iter().fold(HashSet::new(), |mut acc, pos| {
        let neighbours = get_neighbours(map, *pos);
        acc.extend(&neighbours);
        acc
    })
}

fn part1(input: &str, steps: u32) -> usize {
    let (start, map) = parse(input);

    let mut cells = [start].into();
    (0..steps).for_each(|_| {
        cells = evolve(&cells, &map);
        cells.len();
    });

    cells.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../../example1.txt");
        assert_eq!(part1(&input, 6), 16);
    }
}
