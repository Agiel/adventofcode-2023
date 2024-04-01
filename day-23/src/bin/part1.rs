use aocd::*;

#[aocd(2023, 23)]
fn main() {
    let input = input!();
    let result = part1(&input);
    dbg!(result);
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_paths(
    grid: &Vec<Vec<char>>,
    pos: (i32, i32),
    end: (i32, i32),
    prev: (i32, i32),
    length: usize,
) -> Vec<usize> {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .iter()
        .filter_map(|dir| {
            let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
            if new_pos.0 == prev.0 && new_pos.1 == prev.1 || new_pos.0 < 0 || new_pos.1 < 0 {
                return None;
            }
            if new_pos.0 == end.0 && new_pos.1 == end.1 {
                return Some(vec![length + 1]);
            }
            match grid[new_pos.0 as usize][new_pos.1 as usize] {
                '#' => None,
                '.' => Some(find_paths(grid, new_pos, end, pos, length + 1)),
                c => {
                    if dir.0 == 1 && c == '^'
                        || dir.0 == -1 && c == 'v'
                        || dir.1 == 1 && c == '<'
                        || dir.1 == -1 && c == '>'
                    {
                        None
                    } else {
                        Some(find_paths(grid, new_pos, end, pos, length + 1))
                    }
                }
            }
        })
        .flatten()
        .collect()
}

fn part1(input: &str) -> usize {
    let grid = parse(input);

    *find_paths(
        &grid,
        (0, 1),
        (grid.len() as i32 - 1, grid[0].len() as i32 - 2),
        (0, 0),
        0,
    )
    .iter()
    .max()
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../../example1.txt");
        assert_eq!(part1(&input), 94);
    }
}
