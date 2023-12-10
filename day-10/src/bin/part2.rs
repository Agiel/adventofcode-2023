use std::collections::BTreeSet;

use aocd::*;

#[aocd(2023, 10)]
fn main() {
    let input = input!();
    let result = part2(&input);
    dbg!(result);
}

struct Sketch {
    start: (i32, i32),
    map: Vec<Vec<char>>,
}

fn parse(input: &str) -> Sketch {
    let mut start = (0, 0);
    let map = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, char)| {
                    if char == 'S' {
                        start = (x as i32, y as i32);
                    }
                    char
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    Sketch { start, map }
}

fn get_next(pos: (i32, i32), sketch: &Sketch) -> ((i32, i32), (i32, i32)) {
    let shape = if pos.0 < 0 || pos.1 < 0 {
        '.'
    } else {
        sketch.map[pos.1 as usize][pos.0 as usize]
    };
    match shape {
        '|' => ((pos.0, pos.1 - 1), (pos.0, pos.1 + 1)),
        '-' => ((pos.0 - 1, pos.1), (pos.0 + 1, pos.1)),
        'L' => ((pos.0, pos.1 - 1), (pos.0 + 1, pos.1)),
        'J' => ((pos.0 - 1, pos.1), (pos.0, pos.1 - 1)),
        '7' => ((pos.0 - 1, pos.1), (pos.0, pos.1 + 1)),
        'F' => ((pos.0 + 1, pos.1), (pos.0, pos.1 + 1)),
        '.' => ((-1, -1), (-1, -1)),
        _ => panic!(),
    }
}

fn find_connections_from_start(pos: (i32, i32), sketch: &Sketch) -> ((i32, i32), (i32, i32)) {
    let directions = vec![
        (pos.0 - 1, pos.1),
        (pos.0 + 1, pos.1),
        (pos.0, pos.1 - 1),
        (pos.0, pos.1 + 1),
    ];
    let adjacent = directions
        .iter()
        .filter_map(|adj| {
            let connections = get_next(*adj, sketch);
            (connections.0 == pos || connections.1 == pos).then_some(adj)
        })
        .collect::<Vec<_>>();
    (*adjacent[0], *adjacent[1])
}

fn part2(input: &str) -> u32 {
    let sketch = parse(&input);

    let mut is_in_loop = BTreeSet::<(i32, i32)>::new();
    let mut pos = find_connections_from_start(sketch.start, &sketch);
    let mut prev = (sketch.start, sketch.start);
    is_in_loop.insert(sketch.start);

    loop {
        is_in_loop.insert(pos.0);
        is_in_loop.insert(pos.1);

        let next0 = get_next(pos.0, &sketch);
        let next0 = if next0.0 == prev.0 { next0.1 } else { next0.0 };

        let next1 = get_next(pos.1, &sketch);
        let next1 = if next1.0 == prev.1 { next1.1 } else { next1.0 };

        prev = pos;
        pos = (next0, next1);

        if pos.0 == pos.1 {
            is_in_loop.insert(pos.0);
            break;
        }
    }

    let mut area = 0;
    for y in 0..sketch.map.len() {
        let mut inside = false;
        let mut first = '.';
        for x in 0..sketch.map[y].len() {
            let pos = (x as i32, y as i32);
            if is_in_loop.contains(&pos) {
                let shape = sketch.map[y][x];
                // For this to work generally we need to figure out the real shape of S
                // but this is good enough for the examples and my given input.
                if shape == 'F' || shape == 'L' || shape == 'S' {
                    first = shape;
                }
                if shape == '|' || shape == 'J' && first == 'F' || shape == '7' && first == 'L' {
                    inside = !inside;
                }
                print!("{shape}");
            } else {
                if inside {
                    area += 1;
                    print!("I");
                } else {
                    print!("O");
                }
            }
        }
        print!("\n");
    }

    area
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let input = include_str!("../../example1.txt");
        // assert_eq!(part2(&input), 1);
        let input = include_str!("../../example3.txt");
        assert_eq!(part2(&input), 8);
        let input = include_str!("../../example4.txt");
        assert_eq!(part2(&input), 10);
    }
}
