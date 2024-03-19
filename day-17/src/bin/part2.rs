use std::{
    cmp::Ordering,
    collections::{BTreeSet, BinaryHeap},
};

use aocd::*;

#[aocd(2023, 17)]
fn main() {
    let input = input!();
    let result = part2(&input);
    dbg!(result);
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

#[derive(PartialEq, Eq)]
struct State {
    pos: (i32, i32),
    dir: (i32, i32),
    heat: u32,
    same: u8,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .heat
            .cmp(&self.heat)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_path(grid: &Vec<Vec<u32>>, start: (i32, i32), goal: (i32, i32)) -> u32 {
    let mut heap = BinaryHeap::new();
    heap.push(State {
        pos: start,
        dir: (0, 0),
        heat: 0,
        same: 0,
    });

    let mut seen = BTreeSet::new();

    while let Some(state) = heap.pop() {
        if state.pos == goal {
            return state.heat;
        }

        if seen.contains(&(state.pos, state.dir, state.same)) {
            continue;
        }

        seen.insert((state.pos, state.dir, state.same));

        [(1, 0), (0, 1), (-1, 0), (0, -1)].iter().for_each(|dir| {
            if *dir == (-state.dir.0, -state.dir.1) {
                return;
            }

            let new_pos = (state.pos.0 + dir.0, state.pos.1 + dir.1);
            if new_pos.0 < 0
                || new_pos.1 < 0
                || new_pos.0 >= grid.len() as i32
                || new_pos.1 >= grid[0].len() as i32
            {
                return;
            }

            if *dir == state.dir {
                if state.same < 10 {
                    heap.push(State {
                        pos: (state.pos.0 + dir.0, state.pos.1 + dir.1),
                        dir: *dir,
                        heat: state.heat + grid[new_pos.0 as usize][new_pos.1 as usize],
                        same: state.same + 1,
                    });
                }
            } else if state.same == 0 || state.same > 3 {
                heap.push(State {
                    pos: (state.pos.0 + dir.0, state.pos.1 + dir.1),
                    dir: *dir,
                    heat: state.heat + grid[new_pos.0 as usize][new_pos.1 as usize],
                    same: 1,
                });
            }
        })
    }

    0
}

fn part2(input: &str) -> u32 {
    let grid = parse(input);

    return find_path(
        &grid,
        (0, 0),
        (grid.len() as i32 - 1, grid[0].len() as i32 - 1),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../../example1.txt");
        assert_eq!(part2(&input), 94);
    }
}
