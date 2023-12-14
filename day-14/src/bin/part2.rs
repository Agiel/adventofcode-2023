use std::{collections::BTreeMap, time::Instant};

use aocd::*;

#[aocd(2023, 14)]
fn main() {
    let input = input!();
    let start_time = Instant::now();
    let result = part2(&input, 1_000_000_000);
    println!("Elapsed time: {:?}", start_time.elapsed());
    dbg!(result);
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Shape {
    Cube,
    Round,
    None,
}

fn parse(input: &str) -> Vec<Vec<Shape>> {
    let num_rows = input.lines().count();
    let num_cols = input.find(char::is_whitespace).unwrap();
    let mut map = vec![vec![Shape::None; num_rows]; num_cols];
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, shape)| {
            match shape {
                '#' => map[x][y] = Shape::Cube,
                'O' => map[x][y] = Shape::Round,
                _ => (),
            };
        })
    });

    map
}

fn calculate_load(map: &Vec<Vec<Shape>>, num_cols: usize) -> usize {
    map.iter()
        .map(|x| {
            x.iter().enumerate().fold(0, |acc, (y, shape)| match shape {
                Shape::Round => acc + num_cols - y,
                Shape::Cube => acc,
                _ => acc,
            })
        })
        .sum()
}

fn part2(input: &str, spins: usize) -> usize {
    let mut map = parse(input);
    let num_rows = input.lines().count();
    let num_cols = input.find(char::is_whitespace).unwrap();
    let mut results = BTreeMap::new();
    let (start, cycle_length) = (0..spins)
        .find_map(|i| {
            // Tilt north
            (0..num_cols).for_each(|x| {
                let mut last = 0;
                (0..num_rows).for_each(|y| match map[x][y] {
                    Shape::Round => {
                        if y != last {
                            map[x][y] = Shape::None;
                            map[x][last] = Shape::Round;
                        }
                        last += 1;
                    }
                    Shape::Cube => {
                        last = y + 1;
                    }
                    _ => (),
                })
            });
            // Tilt west
            (0..num_rows).for_each(|y| {
                let mut last = 0;
                (0..num_cols).for_each(|x| match map[x][y] {
                    Shape::Round => {
                        if x != last {
                            map[x][y] = Shape::None;
                            map[last][y] = Shape::Round;
                        }
                        last += 1;
                    }
                    Shape::Cube => {
                        last = x + 1;
                    }
                    _ => (),
                })
            });
            // Tilt south
            (0..num_cols).for_each(|x| {
                let mut last = num_rows - 1;
                (0..num_rows).rev().for_each(|y| match map[x][y] {
                    Shape::Round => {
                        if y != last {
                            map[x][y] = Shape::None;
                            map[x][last] = Shape::Round;
                        }
                        last -= 1;
                    }
                    Shape::Cube => {
                        last = y - 1;
                    }
                    _ => (),
                })
            });
            // Tilt east
            (0..num_rows).for_each(|y| {
                let mut last = num_cols - 1;
                (0..num_cols).rev().for_each(|x| match map[x][y] {
                    Shape::Round => {
                        if x != last {
                            map[x][y] = Shape::None;
                            map[last][y] = Shape::Round;
                        }
                        last -= 1;
                    }
                    Shape::Cube => {
                        last = x - 1;
                    }
                    _ => (),
                })
            });

            if let Some(&(first, _)) = results.get(&map) {
                return Some((first, i - first));
            } else {
                let result = calculate_load(&map, num_cols);
                results.insert(map.clone(), (i, result));
            }
            None
        })
        .unwrap();
    let index = start + (spins - start) % cycle_length - 1;
    results
        .values()
        .find_map(|&(i, load)| if i == index { Some(load) } else { None })
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../../example1.txt");
        assert_eq!(part2(&input, 1_000_000_000), 64);
    }
}
