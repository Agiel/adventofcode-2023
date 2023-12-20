use std::{collections::HashMap, ops::Deref};

use aocd::*;
use regex::Regex;

#[aocd(2023, 19)]
fn main() {
    let input = input!();
    let result = part2(&input);
    dbg!(result);
}

#[derive(Clone, Debug)]
struct Part {
    attributes: HashMap<char, (u32, u32)>,
}

#[derive(Debug)]
enum Condition {
    Lt(char, u32, String),
    Gt(char, u32, String),
    Default(String),
}

#[derive(Debug)]
struct Workflow {
    conditions: Vec<Condition>,
}

fn parse_workflows(workflows: &str) -> HashMap<&str, Workflow> {
    let mut map = HashMap::new();
    let re = Regex::new(r"([^\s]+)\{(.+)\}").unwrap();
    let cond_re = Regex::new(r"(\w+)(>|<)(\d+):(\w+)").unwrap();
    re.captures_iter(workflows)
        .map(|c| c.extract())
        .for_each(|(_, [name, conditions])| {
            let conditions = conditions
                .split(",")
                .map(|cond| {
                    if let Some(captures) = cond_re.captures(cond) {
                        match &captures[2] {
                            ">" => Condition::Gt(
                                captures[1].chars().next().unwrap(),
                                captures[3].parse().unwrap(),
                                captures[4].to_string(),
                            ),
                            "<" => Condition::Lt(
                                captures[1].chars().next().unwrap(),
                                captures[3].parse().unwrap(),
                                captures[4].to_string(),
                            ),
                            _ => panic!(),
                        }
                    } else {
                        Condition::Default(cond.to_string())
                    }
                })
                .collect();
            map.insert(name, Workflow { conditions });
        });
    map
}

fn parse(input: &str) -> HashMap<&str, Workflow> {
    let (workflows, _parts) = input.split_once("\n\n").unwrap();
    parse_workflows(workflows)
}

fn run_workflow(
    parts: Vec<Part>,
    workflow: &Workflow,
    workflows: &HashMap<&str, Workflow>,
) -> Vec<Part> {
    let mut accepted = Vec::<Part>::new();
    let mut check = parts;
    workflow.conditions.iter().for_each(|cond| {
        let mut passing = Vec::new();
        let mut keep = Vec::new();
        let goto = match cond {
            Condition::Lt(attr, value, goto) => {
                check.iter().for_each(|p| {
                    let &(min, max) = p.attributes.get(attr).unwrap();
                    if min < *value {
                        let mut p = p.clone();
                        p.attributes.insert(*attr, (min, max.min(value - 1)));
                        passing.push(p);
                    }
                    if max > *value {
                        let mut p = p.clone();
                        p.attributes.insert(*attr, (*value, max));
                        keep.push(p);
                    }
                });
                goto.deref()
            }
            Condition::Gt(attr, value, goto) => {
                check.iter().for_each(|p| {
                    let &(min, max) = p.attributes.get(attr).unwrap();
                    if max > *value {
                        let mut p = p.clone();
                        p.attributes.insert(*attr, (min.max(value + 1), max));
                        passing.push(p);
                    }
                    if min < *value {
                        let mut p = p.clone();
                        p.attributes.insert(*attr, (min, *value));
                        keep.push(p);
                    }
                });
                goto.deref()
            }
            Condition::Default(goto) => {
                passing.append(&mut check);
                goto.deref()
            }
        };

        if passing.len() > 0 {
            match goto {
                "A" => accepted.append(&mut passing),
                "R" => (),
                _ => accepted.append(&mut run_workflow(
                    passing,
                    workflows.get(goto).unwrap(),
                    workflows,
                )),
            }
        }

        check = keep;
    });

    accepted
}

fn part2(input: &str) -> u64 {
    let workflows = parse(input);
    let start = workflows.get("in").unwrap();

    let ranges = run_workflow(
        vec![Part {
            attributes: HashMap::from([
                ('x', (1, 4000)),
                ('m', (1, 4000)),
                ('a', (1, 4000)),
                ('s', (1, 4000)),
            ]),
        }],
        &start,
        &workflows,
    );

    ranges
        .iter()
        .map(|part| {
            part.attributes
                .values()
                .map(|(min, max)| (max - min + 1) as u64)
                .product::<u64>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../../example1.txt");
        assert_eq!(part2(&input), 167409079868000);
    }
}
