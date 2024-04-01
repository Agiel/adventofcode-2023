use std::{
    collections::{BTreeMap, BTreeSet},
    time::Instant,
};

use aocd::*;

#[aocd(2023, 23)]
fn main() {
    let input = input!();
    let start_time = Instant::now();
    let result = part2(&input);
    println!("Elapsed: {:?}", start_time.elapsed());
    dbg!(result);
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[derive(Debug)]
struct Path {
    length: u32,
    end: (i32, i32),
    dir: (i32, i32),
}

fn measure_path(
    grid: &Vec<Vec<char>>,
    start: (i32, i32),
    dir: (i32, i32),
) -> (u32, (i32, i32), (i32, i32)) {
    let mut pos = start;
    let mut dir = dir;
    let mut length = 1;
    loop {
        if let Some((new_pos, new_dir, oob)) =
            [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .iter()
                .find_map(|new_dir| {
                    if new_dir.0 == -dir.0 && new_dir.1 == -dir.1 {
                        return None;
                    }
                    let new_pos = (pos.0 + new_dir.0, pos.1 + new_dir.1);
                    if new_pos.0 < 0
                        || new_pos.1 < 0
                        || new_pos.0 > grid.len() as i32 - 1
                        || new_pos.1 > grid[0].len() as i32 - 1
                    {
                        return Some((new_pos, new_dir, true));
                    }
                    if grid[new_pos.0 as usize][new_pos.1 as usize] == '#' {
                        return None;
                    }
                    Some((new_pos, new_dir, false))
                })
        {
            if oob {
                return (length, pos, *new_dir);
            }
            pos = new_pos;
            dir = *new_dir;
            length += 1;
            if grid[new_pos.0 as usize][new_pos.1 as usize] != '.' {
                return (length, pos, dir);
            }
        }
    }
}

fn find_paths(
    grid: &Vec<Vec<char>>,
    start: (i32, i32),
    dir: (i32, i32),
    paths: &mut BTreeMap<(i32, i32), Path>,
) {
    let (length, end, new_dir) = measure_path(grid, start, dir);
    paths.insert(
        start,
        Path {
            length,
            end,
            dir: new_dir,
        },
    );
    paths.insert(
        end,
        Path {
            length,
            end: start,
            dir: (-dir.0, -dir.1),
        },
    );

    let dir = new_dir;
    let pos = (end.0 + dir.0, end.1 + dir.1);

    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .iter()
        .for_each(|new_dir| {
            if new_dir.0 == -dir.0 && new_dir.1 == -dir.1 {
                return;
            }
            let new_pos = (pos.0 + new_dir.0, pos.1 + new_dir.1);
            if new_pos.0 < 0
                || new_pos.1 < 0
                || new_pos.0 > grid.len() as i32 - 1
                || new_pos.1 > grid[0].len() as i32 - 1
            {
                return;
            }

            let c = grid[new_pos.0 as usize][new_pos.1 as usize];
            if c == '#' || paths.contains_key(&new_pos) {
                return;
            } else {
                find_paths(grid, new_pos, *new_dir, paths);
            }
        })
}

fn find_longest(
    pos: (i32, i32),
    length: u32,
    end: (i32, i32),
    paths: &BTreeMap<(i32, i32), Path>,
    visited: &mut BTreeSet<(i32, i32)>,
) -> Option<u32> {
    if let Some(path) = paths.get(&pos) {
        let length = length + path.length;
        let pos = path.end;
        if pos.0 == end.0 && pos.1 == end.1 {
            return Some(length);
        }
        // Go to node
        let dir = path.dir;
        let pos = (pos.0 + dir.0, pos.1 + dir.1);
        let length = length + 1;
        if visited.contains(&pos) {
            return None;
        }
        visited.insert(pos);
        let length = [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            .filter_map(|new_dir| {
                if new_dir.0 == -dir.0 && new_dir.1 == -dir.1 {
                    return None;
                }
                let new_pos = (pos.0 + new_dir.0, pos.1 + new_dir.1);
                find_longest(new_pos, length, end, paths, visited)
            })
            .max();
        visited.remove(&pos);
        return length;
    }
    None
}

fn part2(input: &str) -> u32 {
    let grid = parse(input);

    let mut paths = BTreeMap::new();
    find_paths(&grid, (0, 1), (1, 0), &mut paths);

    find_longest(
        (0, 1),
        0,
        (grid.len() as i32 - 1, grid[0].len() as i32 - 2),
        &paths,
        &mut BTreeSet::new(),
    )
    .unwrap_or(0)
        - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../../example1.txt");
        assert_eq!(part2(&input), 154);
    }
}
