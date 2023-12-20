//! YEAR:   2023
//! DAY:    19

use itertools::{iproduct, Itertools};
use rayon::iter::{IntoParallelRefMutIterator, ParallelBridge};
use rayon::prelude::ParallelIterator;
use std::collections::{HashMap, HashSet};

use crate::prelude::*;
use crate::regex::regex;

impl Answers for Day {
    fn part_one(&mut self) -> String {
        let mut sort_system = SortSystem::new(&self.input);
        sort_system.rootin_scootin_sortin();
        sort_system.find_accepted().to_string()
    }

    fn part_two(&mut self) -> String {
        let sort_system = SortSystem::new(&self.input);
        sort_system.find_accept_conditions().to_string()
    }
}

#[derive(Debug)]
struct SortSystem {
    workflows: HashMap<String, Workflow>,
    parts: Vec<Part>,
}

#[derive(Debug)]
struct Workflow {
    xmas: Vec<XmasFlow>,
    other: String,
}

#[derive(Debug)]
struct Part {
    xmas: HashMap<char, usize>,
    res: String,
}
#[derive(Debug)]
struct XmasFlow {
    cat: char,
    cmp: char,
    val: usize,
    out: String,
}
#[derive(Clone)]
struct RatingRange {
    xmas: HashMap<char, (usize, usize)>,
    res: String,
}

impl XmasFlow {
    fn compare(&self, input: &usize) -> bool {
        match self.cmp {
            '>' => return *input > self.val,
            '<' => return *input < self.val,
            _ => false,
        }
    }

    fn pass_range(&self, range: &RatingRange) -> Result<RatingRange, bool> {
        let mut pass_range = range.clone();
        pass_range.res = self.out.to_string();
        let nr_cat = pass_range.xmas.get_mut(&self.cat).unwrap();
        match self.cmp {
            '>' => *nr_cat = (self.val + 1, nr_cat.1),
            '<' => *nr_cat = (nr_cat.0, self.val - 1),
            _ => unreachable!(),
        }
        if nr_cat.1 >= nr_cat.0 {
            Ok(pass_range)
        } else {
            Err(false)
        }
    }

    fn fail_range(&self, range: &mut RatingRange) {
        let nr_cat = range.xmas.get_mut(&self.cat).unwrap();
        match self.cmp {
            '>' => *nr_cat = (nr_cat.0, self.val),
            '<' => *nr_cat = (self.val, nr_cat.1),
            _ => unreachable!(),
        }
    }
}

impl SortSystem {
    fn new(input: &'_ str) -> Self {
        let mut lines = input.lines();
        let mut line = lines.next().unwrap();
        let mut sort_system = SortSystem {
            workflows: HashMap::new(),
            parts: vec![],
        };

        let workflows_finder = regex("2318_workflow");
        let xmas_finder = regex("2318_xmas");
        while !line.is_empty() {
            let groups = workflows_finder.captures(line).unwrap().unwrap();
            sort_system.workflows.insert(
                groups["name"].to_string(),
                Workflow {
                    xmas: xmas_finder
                        .captures_iter(&groups["xmas"])
                        .map(|x| match x {
                            Ok(x) => XmasFlow {
                                cat: x["xmas"].chars().next().unwrap(),
                                cmp: x["cmp"].chars().next().unwrap(),
                                val: x["val"].parse::<usize>().unwrap(),
                                out: x["out"].to_string(),
                            },
                            Err(e) => panic!("{e}"),
                        })
                        .collect_vec(),
                    other: groups["other"].to_string(),
                },
            );
            line = lines.next().unwrap()
        }
        line = lines.next().unwrap();
        let part_finder = regex("2318_part_ratings");
        while !line.is_empty() {
            sort_system.parts.push(Part {
                xmas: part_finder
                    .captures_iter(line)
                    .map(|c| {
                        let cap = c.unwrap();
                        return (
                            cap["xmas"].chars().next().unwrap(),
                            cap["val"].parse::<usize>().unwrap(),
                        );
                    })
                    .collect::<HashMap<char, usize>>(),
                res: "in".to_string(),
            });
            line = lines.next().unwrap_or("");
        }
        sort_system
    }

    fn rootin_scootin_sortin(&mut self) {
        self.parts
            .par_iter_mut()
            .for_each(|p| run_workflow(&self.workflows, p))
    }

    // I just wanted to see if this works. Suprise surprise it aint it chief.
    #[allow(unused)]
    fn oonga_boonga_combo(&self) -> usize {
        iproduct!(1..=4000, 1..=4000, 1..=4000, 1..=4000)
            .par_bridge()
            .map(|(x, m, a, s)| {
                let mut part = Part {
                    xmas: vec![('x', x), ('m', m), ('a', a), ('s', s)]
                        .into_iter()
                        .collect::<HashMap<char, usize>>(),
                    res: "in".to_string(),
                };
                run_workflow(&self.workflows, &mut part);
                if part.res == "A" {
                    1
                } else {
                    0
                }
            })
            .sum()
    }

    fn find_accepted(&self) -> usize {
        self.parts
            .iter()
            .map(|p| {
                if p.res == "A" {
                    return p.xmas.values().sum();
                } else {
                    0
                }
            })
            .sum()
    }
    fn find_accept_conditions(&self) -> usize {
        let range = RatingRange {
            xmas: ['x', 'm', 'a', 's']
                .into_iter()
                .map(|c| (c, (1, 4000)))
                .collect::<HashMap<char, (usize, usize)>>(),
            res: "in".to_string(),
        };
        let mut memo = HashSet::new();
        let mut results = vec![];
        self.find_connecting_workflows(&mut memo, &mut results, range);
        results.iter().map(|r| calc_combinations(r)).sum()
    }

    fn find_connecting_workflows(
        &self,
        memo: &mut HashSet<(String, String)>,
        results: &mut Vec<RatingRange>,
        range: RatingRange,
    ) {
        if range.res == "A" {
            results.push(range)
        } else {
            let workflow = self.workflows.get(&range.res).unwrap();
            // If someone has a better idea than this, let me know. I wanted to use a HashMap because
            // the number of checks per workflow was dynamic and sometimes there were several of the
            // same category. I wasn't sure if I wanted to do a big match statement.
            let hash_input = (
                range.res.to_string(),
                range
                    .xmas
                    .iter()
                    .map(|(c, (x, y))| format!("{}({},{})", c, x, y))
                    .collect(),
            );
            if memo.get(&hash_input).is_none() {
                memo.insert(hash_input);
                let mut new_range = range.clone();
                for xmasflow in &workflow.xmas {
                    if xmasflow.out != "R" {
                        if let Ok(pass_range) = xmasflow.pass_range(&new_range) {
                            self.find_connecting_workflows(memo, results, pass_range)
                        }
                    }
                    xmasflow.fail_range(&mut new_range);
                }
                if workflow.other != "R" {
                    new_range.res = workflow.other.to_string();
                    self.find_connecting_workflows(memo, results, new_range)
                }
            }
        }
    }
}

fn calc_combinations(range: &RatingRange) -> usize {
    range.xmas.values().fold(1, |acc, r| acc * (r.1 - r.0 + 1))
}

fn run_workflow(workflows: &HashMap<String, Workflow>, part: &mut Part) {
    '_wkf: while part.res != "A" && part.res != "R" {
        let workflow = workflows.get(&part.res).unwrap();
        for xmasflow in &workflow.xmas {
            if let Some(val) = part.xmas.get(&xmasflow.cat) {
                if xmasflow.compare(val) {
                    part.res = xmasflow.out.clone();
                    continue '_wkf;
                }
            }
        }
        part.res = workflow.other.clone();
    }
}
