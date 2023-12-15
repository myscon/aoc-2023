//! YEAR:   2023
//! DAY:    11

use crate::{prelude::*, regex::regex};
use itertools::Itertools;

impl Answers for Day {
    fn part_one(&mut self) -> Result<String, Box<dyn Error>> {
        let universe = Universe::new(&self.input, 1);
        Ok(universe.gen_combinations().to_string().into())
    }

    fn part_two(&mut self) -> Result<String, Box<dyn Error>> {
        let universe = Universe::new(&self.input, 999999);
        Ok(universe.gen_combinations().to_string().into())
    }
}

#[derive(Debug)]
struct Universe {
    observable: Vec<(usize, usize)>,
}

trait Telescope {
    fn new(input: &str, expansion: usize) -> Self;
    fn gen_combinations(&self) -> usize;
}

impl Telescope for Universe {
    fn new(input: &str, expansion: usize) -> Self {
        let galaxy_finder = regex("galaxy");
        let univ_len = input.lines().next().unwrap().len();
        let mut universe = vec![];
        let mut empty_cols = vec![0 as usize; univ_len];
        // I'm tired of doing buffers so I'm saving the input as a whole then processing it
        let mut line_num = 0 as usize;
        for line in input.lines() {
            let gline = galaxy_finder
                .find_iter(line)
                .map(|g| (line_num, g.unwrap().start()))
                .collect::<Vec<(usize, usize)>>();
            if gline.len() == 0 {
                line_num += expansion
            } else {
                for g in &gline {
                    empty_cols[g.1] += 1;
                }
                universe.extend(gline);
            }
            line_num += 1
        }
        // what is this O(∞)?
        let mut emptyc = 0;
        for i in 0..empty_cols.len() {
            if empty_cols[i] == 0 {
                for g in &mut universe {
                    if g.1 > i + emptyc {
                        g.1 += expansion;
                    }
                }
                emptyc += expansion;
            }
        }
        Universe {
            observable: universe,
        }
    }

    fn gen_combinations(&self) -> usize{
        let combinations = self.observable.iter().combinations(2);
        let mut accumulator = 0 as usize;
        for comb in combinations {
            let diff = comb[0].0.abs_diff(comb[1].0) + comb[0].1.abs_diff(comb[1].1);
            accumulator += diff
        }
        accumulator
    }
}
