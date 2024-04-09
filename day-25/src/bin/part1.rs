use std::collections::{HashMap, HashSet, VecDeque};

use aocd::*;

#[aocd(2023, 25)]
fn main() {
    let input = input!();
    let result = part1(&input);
    dbg!(result);
}

fn parse(input: &str) -> HashMap<&str, Vec<String>> {
    let mut graph = HashMap::<&str, Vec<String>>::new();
    input.lines().for_each(|line| {
        if let Some((node, neighbours)) = line.split_once(": ") {
            neighbours.split(" ").for_each(|neighbour| {
                graph
                    .entry(node)
                    .and_modify(|e| e.push(neighbour.to_string()))
                    .or_insert(vec![neighbour.to_string()]);
                graph
                    .entry(neighbour)
                    .and_modify(|e| e.push(node.to_string()))
                    .or_insert(vec![node.to_string()]);
            })
        }
    });

    graph
}

fn find_busiest(graph: &HashMap<&str, Vec<String>>) -> Vec<(String, String)> {
    let mut edge_counts = HashMap::<(&str, &str), u32>::new();

    graph.keys().for_each(|node| {
        let mut to_visit = VecDeque::from([*node]);
        let mut seen = HashSet::from([*node]);
        let mut edges = HashMap::<&str, &str>::new();
        while let Some(visiting) = to_visit.pop_front() {
            let mut added = 0;
            graph.get(visiting).unwrap().iter().for_each(|n| {
                if !seen.contains(n.as_str()) {
                    seen.insert(n);
                    edges.insert(n, visiting);
                    to_visit.push_back(n);
                    added += 1;
                }
            });

            // Small optimization by only tracing paths between nodes on opposite ends of the graph
            if added > 0 {
                continue;
            }

            // Count
            let mut current = visiting;
            while let Some(&parent) = edges.get(current) {
                let key = if parent < current {
                    (parent, current)
                } else {
                    (current, parent)
                };
                edge_counts
                    .entry(key)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
                current = parent;
            }
        }
    });

    let mut edge_counts = edge_counts.iter().collect::<Vec<_>>();
    edge_counts.sort_by(|a, b| b.1.cmp(a.1));
    edge_counts
        .iter()
        .take(3)
        .map(|e| (e.0 .0.to_string(), e.0 .1.to_string()))
        .collect()
}

fn count_groups(graph: &HashMap<&str, Vec<String>>) -> (usize, usize) {
    let start = graph.keys().next().unwrap();
    let mut to_visit = VecDeque::from([*start]);
    let mut seen = HashSet::from([*start]);
    while let Some(visiting) = to_visit.pop_front() {
        graph.get(visiting).unwrap().iter().for_each(|n| {
            if !seen.contains(n.as_str()) {
                seen.insert(n);
                to_visit.push_back(n);
            }
        });
    }
    (seen.len(), graph.keys().count() - seen.len())
}

fn part1(input: &str) -> usize {
    let mut graph = parse(input);

    let top3 = find_busiest(&graph);
    top3.iter().for_each(|(left, right)| {
        let nodes = graph.get_mut(left.as_str()).unwrap();
        if let Some(pos) = nodes.iter().position(|n| n == right) {
            nodes.remove(pos);
        }
        let nodes = graph.get_mut(right.as_str()).unwrap();
        if let Some(pos) = nodes.iter().position(|n| n == left) {
            nodes.remove(pos);
        }
    });

    let (left, right) = count_groups(&graph);

    left * right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../../example1.txt");
        assert_eq!(part1(&input), 54);
    }
}
