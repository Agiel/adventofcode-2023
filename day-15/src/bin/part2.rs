use aocd::*;

#[aocd(2023, 15)]
fn main() {
    let input = input!();
    let result = part2(&input);
    dbg!(result);
}

#[derive(Debug)]
struct HashMap {
    buckets: Vec<Vec<(String, usize)>>,
}

fn hash(string: &str) -> usize {
    string
        .chars()
        .fold(0, |acc, c| ((acc + c as usize) * 17) % 256)
}

impl HashMap {
    fn new() -> Self {
        Self {
            buckets: vec![vec![]; 256],
        }
    }

    fn remove(&mut self, label: &str) {
        let h = hash(label);
        if let Some(i) = self.buckets[h]
            .iter()
            .enumerate()
            .find_map(|(idx, (lbl, _))| (lbl == label).then_some(idx))
        {
            self.buckets[h].remove(i);
        }
    }

    fn add(&mut self, label: &str, focal_length: usize) {
        let h = hash(label);
        if let Some(i) = self.buckets[h]
            .iter()
            .enumerate()
            .find_map(|(idx, (lbl, _))| (lbl == label).then_some(idx))
        {
            self.buckets[h][i] = (label.to_string(), focal_length);
        } else {
            self.buckets[h].push((label.to_string(), focal_length));
        }
    }

    fn focal_power(&self) -> usize {
        self.buckets
            .iter()
            .enumerate()
            .map(|(id, bucket)| {
                (id + 1)
                    * bucket
                        .iter()
                        .enumerate()
                        .map(|(slot, &(_, focal_length))| (slot + 1) * focal_length)
                        .sum::<usize>()
            })
            .sum()
    }
}

fn part2(input: &str) -> usize {
    let mut hash_map = HashMap::new();
    input.trim().split(',').for_each(|cmd| {
        if cmd.ends_with('-') {
            hash_map.remove(&cmd[..cmd.len()-1]);
        } else {
            let (label, focal_length) = cmd.split_once('=').unwrap();
            hash_map.add(label, focal_length.parse::<usize>().unwrap());
        }
    });
    hash_map.focal_power()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../../example1.txt");
        assert_eq!(part2(&input), 145);
    }
}
