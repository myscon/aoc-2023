//! YEAR:   2023
//! DAY:    20

use itertools::Itertools;
use num_integer::lcm;
use std::collections::{HashMap, HashSet, VecDeque};

use crate::prelude::*;
use crate::regex::regex;

impl Answers for Day {
    fn part_one(&mut self) -> String {
        let mut machine = Machine::new(&self.input);
        machine.pull_the_lever_kronk(true).to_string()
    }

    fn part_two(&mut self) -> String {
        let mut machine = Machine::new(&self.input);
        machine.pull_the_lever_kronk(false).to_string()
    }
}
#[derive(Debug)]
struct Machine {
    modules: HashMap<String, Module>,
    pulse_reqs: usize,
}

// I really wish rust had inheretence. The enums were a pain in the ass for the handful
// of fields.
#[derive(Debug, Clone)]
struct Module {
    mtype: char,
    inputs: HashMap<String, char>,
    outputs: Vec<String>,
    toggle: bool,
    pulse_req: char,
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Pulse {
    pulse: char,
    source: String,
    targets: Vec<String>,
}

impl Machine {
    fn new(input: &str) -> Self {
        let line_r = regex("2320_machines");
        let outputs_r = regex("2320_outputs");
        let mut inputs = HashMap::new();
        let mut machine = Machine {
            modules: input
                .lines()
                .map(|l| {
                    let matches = line_r.captures(l).unwrap().unwrap();
                    let outputs = outputs_r
                        .find_iter(&matches["outputs"])
                        .map(|o| o.unwrap().as_str().to_string())
                        .collect_vec();
                    let name;
                    let mtype;
                    match matches.name("name") {
                        Some(m) => name = m.as_str(),
                        _ => name = "bc",
                    }
                    match matches.name("type") {
                        Some(m) => mtype = m.as_str(),
                        _ => mtype = "$",
                    }
                    outputs.iter().for_each(|o| {
                        inputs
                            .entry(o.to_string())
                            .or_insert(HashMap::new())
                            .insert(name.to_string(), 'l');
                    });
                    (
                        name.to_string(),
                        Module {
                            mtype: mtype.chars().next().unwrap(),
                            inputs: HashMap::new(),
                            outputs: outputs,
                            toggle: false,
                            pulse_req: 'r',
                        },
                    )
                })
                .collect::<HashMap<String, Module>>(),
            pulse_reqs: 0,
        };
        machine.modules.insert(
            "rx".to_string(),
            Module {
                mtype: 'O',
                inputs: HashMap::new(),
                outputs: vec![],
                toggle: false,
                pulse_req: 'l',
            },
        );
        // oh boy I did not like this
        Machine::fill_inputs(&mut machine, inputs);
        Machine::fill_pulse_req(&mut machine);
        machine
    }

    fn fill_inputs(&mut self, inputs: HashMap<String, HashMap<String, char>>) {
        for (name, inputs) in inputs.iter() {
            if let Some(module) = self.modules.get_mut(name) {
                module.inputs = inputs.clone();
            }
        }
    }

    fn fill_pulse_req(&mut self) {
        let modules = self.modules.clone();
        let module = modules.get("rx").unwrap();
        let mut memo = HashSet::new();
        self.fill_pulse_req_helper(&mut memo, &modules, module, module.pulse_req);
        self.pulse_reqs = self
            .modules
            .values()
            .map(|m| {
                if m.pulse_req != 'r' && m.mtype != 'O' {
                    1
                } else {
                    0
                }
            })
            .sum()
    }

    fn fill_pulse_req_helper(
        &mut self,
        memo: &mut HashSet<String>,
        modules: &HashMap<String, Module>,
        module: &Module,
        previous: char,
    ) {
        if module.mtype != '$' {
            let pulse_req = match module.mtype {
                '%' => 'r',
                '&' => match previous {
                    'h' => 'r',
                    'l' => 'h',
                    'r' => 'r',
                    _ => unreachable!(),
                },
                'O' => 'l',
                _ => unreachable!(),
            };
            for (o, _) in &module.inputs {
                if memo.get(o).is_none() {
                    let o_mod = self.modules.get_mut(o).unwrap();
                    o_mod.pulse_req = pulse_req;
                    memo.insert(o.to_string());
                    self.fill_pulse_req_helper(memo, modules, modules.get(o).unwrap(), pulse_req)
                }
            }
        }
    }

    fn pull_the_lever_kronk(&mut self, wrong_lever: bool) -> usize {
        let mut day8_shenanigans = HashMap::new();
        let module_cnt = self.pulse_reqs - 1;
        let mut queue = VecDeque::new();
        let mut presses = 1;
        let mut high_cnt = 0;
        let mut low_cnt = 0;
        press_button(&mut queue);
        '_main: while !queue.is_empty() {
            let pulse = queue.pop_front().unwrap();
            match pulse.pulse {
                'h' => high_cnt += pulse.targets.len(),
                'l' => low_cnt += pulse.targets.len(),
                _ => unreachable!(),
            }
            for target in pulse.targets {
                if let Some(module) = self.modules.get_mut(&target) {
                    if let Some(next_pulse) = module.process_pulse(&pulse.source, pulse.pulse) {
                        if target == "rx" && next_pulse == 'l' {
                            break;
                        }
                        if !wrong_lever && day8_shenanigans.get(&target).is_none() {
                            if next_pulse == module.pulse_req {
                                day8_shenanigans.insert(target.to_string(), presses);
                                if day8_shenanigans.len() == module_cnt {
                                    break '_main;
                                }
                            }
                        }
                        queue.push_back(Pulse {
                            pulse: next_pulse,
                            source: target,
                            targets: module.outputs.clone(),
                        });
                    };
                }
            }
            if queue.is_empty() && (!wrong_lever || presses < 1000) {
                presses += 1;
                press_button(&mut queue);
            }
        }
        if wrong_lever {
            high_cnt * low_cnt
        } else {
            day8_shenanigans.values().fold(1, |acc, &x| lcm(acc, x))
        }
    }
}

impl Module {
    fn process_pulse(&mut self, source: &str, pulse: char) -> Option<char> {
        match self.mtype {
            '%' => match pulse {
                'l' => {
                    self.toggle = !self.toggle;
                    if self.toggle {
                        Some('h')
                    } else {
                        Some('l')
                    }
                }
                _ => None,
            },
            '&' => {
                self.inputs.insert(source.to_string(), pulse).unwrap();
                if self.inputs.values().all(|&p| p == 'h') {
                    Some('l')
                } else {
                    Some('h')
                }
            }
            'O' => match pulse {
                'l' => Some('r'),
                _ => None,
            },
            '$' => Some(pulse),
            _ => unreachable!(),
        }
    }
}

fn press_button(queue: &mut VecDeque<Pulse>) {
    queue.push_back(Pulse {
        pulse: 'l',
        source: "bn".to_string(),
        targets: vec!["bc".to_string()],
    })
}
