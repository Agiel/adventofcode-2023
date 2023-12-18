use aocd::*;

#[aocd(2023, 16)]
fn main() {
    let input = input!();
    let result = part2(&input);
    dbg!(result);
}

fn parse(input: &str) -> Vec<Vec<(char, (bool, bool, bool, bool))>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| (c, (false, false, false, false)))
                .collect()
        })
        .collect()
}

fn trace(
    mut grid: &mut Vec<Vec<(char, (bool, bool, bool, bool))>>,
    pos: (i32, i32),
    direction: (i32, i32),
) {
    if pos.0 < 0 || pos.1 < 0 {
        return;
    }
    let Some(tile) = grid
        .get_mut(pos.1 as usize)
        .and_then(|x| x.get_mut(pos.0 as usize))
    else {
        return;
    };

    if tile.1 .0 && direction.0 > 0
        || tile.1 .1 && direction.0 < 0
        || tile.1 .2 && direction.1 > 0
        || tile.1 .3 && direction.1 < 0
    {
        return;
    }
    tile.1 = (
        tile.1 .0 || direction.0 > 0,
        tile.1 .1 || direction.0 < 0,
        tile.1 .2 || direction.1 > 0,
        tile.1 .3 || direction.1 < 0,
    );
    match tile.0 {
        '|' => {
            if direction.0 != 0 {
                trace(&mut grid, (pos.0, pos.1 + 1), (0, 1));
                trace(&mut grid, (pos.0, pos.1 - 1), (0, -1));
            } else {
                trace(
                    &mut grid,
                    (pos.0 + direction.0, pos.1 + direction.1),
                    direction,
                );
            }
        }
        '-' => {
            if direction.1 != 0 {
                trace(&mut grid, (pos.0 + 1, pos.1), (1, 0));
                trace(&mut grid, (pos.0 - 1, pos.1), (-1, 0));
            } else {
                trace(
                    &mut grid,
                    (pos.0 + direction.0, pos.1 + direction.1),
                    direction,
                );
            }
        }
        '/' => {
            let direction = (-direction.1, -direction.0);
            trace(
                &mut grid,
                (pos.0 + direction.0, pos.1 + direction.1),
                direction,
            );
        }
        '\\' => {
            let direction = (direction.1, direction.0);
            trace(
                &mut grid,
                (pos.0 + direction.0, pos.1 + direction.1),
                direction,
            );
        }
        _ => trace(
            &mut grid,
            (pos.0 + direction.0, pos.1 + direction.1),
            direction,
        ),
    }
}

fn energize(
    mut grid: Vec<Vec<(char, (bool, bool, bool, bool))>>,
    pos: (i32, i32),
    direction: (i32, i32),
) -> u32 {
    trace(&mut grid, pos, direction);

    grid.iter()
        .map(|col| {
            col.iter()
                .filter_map(|tile| (tile.1 .0 || tile.1 .1 || tile.1 .2 || tile.1 .3).then_some(1))
                .sum::<u32>()
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    let grid = parse(input);

    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    let max_rows = (0..rows)
        .map(|row| {
            energize(grid.clone(), (0, row), (1, 0)).max(energize(
                grid.clone(),
                (cols - 1, row),
                (-1, 0),
            ))
        })
        .max()
        .unwrap();
    let max_cols = (0..cols)
        .map(|col| {
            energize(grid.clone(), (col, 0), (0, 1)).max(energize(
                grid.clone(),
                (col, rows - 1),
                (0, -1),
            ))
        })
        .max()
        .unwrap();

    max_rows.max(max_cols)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../../example1.txt");
        assert_eq!(part2(&input), 51);
    }
}
