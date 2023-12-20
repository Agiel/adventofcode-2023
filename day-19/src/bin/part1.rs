use std::{collections::HashMap, ops::Deref};

use aocd::*;
use regex::Regex;

#[aocd(2023, 19)]
fn main() {
    let input = input!();
    let result = part1(&input);
    dbg!(result);
}

struct Part {
    attributes: HashMap<char, u32>,
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

fn parse_parts(parts: &str) -> Vec<Part> {
    parts
        .lines()
        .map(|part| {
            let part = &part[1..part.len() - 1];
            let attributes = HashMap::from_iter(part.split(",").map(|attribute| {
                let (name, value) = attribute.split_once("=").unwrap();
                let name = name.chars().next().unwrap();
                let value = value.parse().unwrap();
                (name, value)
            }));
            Part { attributes }
        })
        .collect()
}

fn parse(input: &str) -> (Vec<Part>, HashMap<&str, Workflow>) {
    let (workflows, parts) = input.split_once("\n\n").unwrap();
    let workflows = parse_workflows(workflows);
    let parts = parse_parts(parts);
    (parts, workflows)
}

fn run_workflow(part: &Part, workflow: &Workflow, workflows: &HashMap<&str, Workflow>) -> bool {
    let goto = workflow
        .conditions
        .iter()
        .find_map(|cond| match cond {
            Condition::Lt(attr, value, goto) => {
                (part.attributes.get(attr).unwrap() < value).then_some(goto)
            }
            Condition::Gt(attr, value, goto) => {
                (part.attributes.get(attr).unwrap() > value).then_some(goto)
            }
            Condition::Default(goto) => Some(goto),
        })
        .unwrap();

    match goto.deref() {
        "A" => true,
        "R" => false,
        _ => run_workflow(part, workflows.get(goto.deref()).unwrap(), workflows),
    }
}

fn part1(input: &str) -> u32 {
    let (parts, workflows) = parse(input);
    let start = workflows.get("in").unwrap();

    parts
        .iter()
        .filter_map(|part| {
            run_workflow(&part, &start, &workflows).then_some(part.attributes.values().sum::<u32>())
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../../example1.txt");
        assert_eq!(part1(&input), 19114);
    }
}
