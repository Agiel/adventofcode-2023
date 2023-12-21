use std::collections::HashMap;

use aocd::*;
use num::integer::lcm;

#[aocd(2023, 20)]
fn main() {
    let input = input!();
    let result = part2(&input);
    dbg!(result);
}

enum ModuleType {
    Broadcaster,
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
}

struct Module {
    name: String,
    module_type: ModuleType,
    outputs: Vec<String>,
}

struct Pulse {
    high: bool,
    source: String,
    destination: String,
}

impl Module {
    fn process(&mut self, input: &Pulse) -> Vec<Pulse> {
        match &mut self.module_type {
            ModuleType::Broadcaster => self
                .outputs
                .iter()
                .map(|o| Pulse {
                    high: input.high,
                    source: self.name.clone(),
                    destination: o.clone(),
                })
                .collect(),
            ModuleType::FlipFlop(on) => {
                if !input.high {
                    *on = !*on;
                    self.outputs
                        .iter()
                        .map(|o| Pulse {
                            high: *on,
                            source: self.name.clone(),
                            destination: o.clone(),
                        })
                        .collect()
                } else {
                    Vec::new()
                }
            }
            ModuleType::Conjunction(inputs) => {
                inputs.insert(input.source.clone(), input.high);
                let high = !inputs.values().all(|i| *i);
                self.outputs
                    .iter()
                    .map(|o| Pulse {
                        high,
                        source: self.name.clone(),
                        destination: o.clone(),
                    })
                    .collect()
            }
        }
    }
}

fn parse(input: &str) -> HashMap<String, Module> {
    let mut modules: HashMap<_, _> = input
        .lines()
        .map(|line| {
            let (mut name, outputs) = line.split_once(" -> ").unwrap();
            let module_type = match name.chars().next().unwrap() {
                'b' => ModuleType::Broadcaster,
                '%' => {
                    name = &name[1..];
                    ModuleType::FlipFlop(false)
                }
                '&' => {
                    name = &name[1..];
                    ModuleType::Conjunction(HashMap::new())
                }
                _ => panic!(),
            };
            let name = name.to_string();
            (
                name.clone(),
                Module {
                    name,
                    module_type,
                    outputs: outputs.split(", ").map(|s| s.to_string()).collect(),
                },
            )
        })
        .collect();

    // Fill conjunction inputs
    let mut inputs = Vec::new();
    modules.values().for_each(|module| {
        module.outputs.iter().for_each(|o| {
            if let Some(Module {
                module_type: ModuleType::Conjunction(_),
                ..
            }) = modules.get(o)
            {
                inputs.push((o.clone(), module.name.clone()));
            }
        })
    });
    inputs.iter().for_each(|o| {
        if let Some(Module {
            module_type: ModuleType::Conjunction(inputs),
            ..
        }) = modules.get_mut(&o.0)
        {
            inputs.insert(o.1.clone(), false);
        }
    });

    modules
}

fn part2(input: &str) -> u64 {
    let mut modules = parse(input);

    let mut presses = 0;

    let mut last_inputs = HashMap::<String, bool>::new();
    let mut cycles = HashMap::<String, u32>::new();

    loop {
        presses += 1;
        let mut buffer = vec![Pulse {
            source: "button".to_string(),
            high: false,
            destination: "broadcaster".to_string(),
        }];
        while !buffer.is_empty() {
            buffer = buffer
                .iter()
                .flat_map(|p| {
                    if let Some(module) = modules.get_mut(&p.destination) {
                        module.process(p)
                    } else {
                        Vec::new()
                    }
                })
                .collect();
            // Note that we hardcode "cl" here as it's the input to "rx". This might differ between puzzle inputs.
            if let Some(Module {
                module_type: ModuleType::Conjunction(inputs),
                ..
            }) = modules.get("cl")
            {
                inputs.iter().for_each(|(k, v)| {
                    if !v && v != last_inputs.get(k).unwrap_or(&false) {
                        cycles.insert(k.clone(), presses);
                        println!("{} : {}", k, presses);
                    }
                });
                if cycles.len() == inputs.len() {
                    return cycles
                        .values()
                        .fold(1, |acc, cycle| lcm(acc, *cycle as u64));
                }
                last_inputs = inputs.clone();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../../example1.txt");
        assert_eq!(part2(&input), 32000000);
        let input = include_str!("../../example2.txt");
        assert_eq!(part2(&input), 11687500);
    }
}
