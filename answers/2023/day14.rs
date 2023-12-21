//! YEAR:   2023
//! DAY:    14

use crate::prelude::*;
use std::collections::HashMap;
use itertools::Itertools;

impl Answers for Day {
    fn part_one(&mut self) -> String {
        let mut dish = Reflector::new(&self.input);
        dish.tilt_vert("north");
        dish.tally_dish().to_string()
    }

    fn part_two(&mut self) -> String {
        let mut dish = Reflector::new(&self.input);
        let mut cycler = HashMap::new();
        let mut idx = 0;
        cycler.insert(dish.dish.iter().map(|v| v.iter()).flatten().collect(), idx);
        while idx < 1000000000 {
            dish.tilt_vert("north");
            dish.tilt_horz("west");
            dish.tilt_vert("south");
            dish.tilt_horz("east");
            let entry = dish.dish.iter().flatten().collect::<String>();
            if cycler.contains_key(&entry) {
                let cycle_idx = cycler.get(&entry).unwrap();
                let diff = idx - cycle_idx;
                idx += ((1000000000 - idx) / diff) * diff // floor divide
            } else {
                cycler.insert(entry, idx);
            }
            idx += 1;
        }
        dish.tally_dish().to_string()
    }
}

struct Reflector {
    original: Vec<Vec<char>>,
    dish: Vec<Vec<char>>,
}

impl Reflector {
    fn new(input: &str) -> Self {
        let dish = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
        Reflector {
            original: dish.clone(),
            dish: dish.clone(),
        }
    }

    fn tilt_vert(&mut self, direction: &str) {
        let mut roll = vec![0; self.dish[0].len()];
        for i in 0..self.dish.len() {
            let row = match direction {
                "north" => i,
                "south" => self.dish.len() - i - 1,
                _ => unreachable!(),
            };
            let (rocks, stones, spaces) = rock_n_stone(&self.dish[row]);
            rocks.iter().for_each(|&r| {
                match direction {
                    "north" => self.dish[row - roll[r]][r] = 'O',
                    "south" => self.dish[row + roll[r]][r] = 'O',
                    _ => unreachable!(),
                }
                if roll[r] > 0 {
                    self.dish[row][r] = '.';
                }
            });
            stones.iter().for_each(|&s| roll[s] = 0);
            spaces.iter().for_each(|&s| roll[s] += 1);
        }
    }

    fn tilt_horz(&mut self, direction: &str) {
        let mut roll = vec![0; self.dish[0].len()];
        for i in 0..self.dish[0].len() {
            let col = match direction {
                "west" => i,
                "east" => self.dish[0].len() - i - 1,
                _ => unreachable!(),
            };
            let (rocks, stones, spaces) =
                rock_n_stone(&self.dish.iter().map(|r| r[col]).collect_vec());
            rocks.iter().for_each(|&r| {
                match direction {
                    "west" => self.dish[r][col - roll[r]] = 'O',
                    "east" => self.dish[r][col + roll[r]] = 'O',
                    _ => unreachable!(),
                }
                if roll[r] > 0 {
                    self.dish[r][col] = '.';
                }
            });
            stones.iter().for_each(|&s| roll[s] = 0);
            spaces.iter().for_each(|&s| roll[s] += 1);
        }
    }

    fn tally_dish(&self) -> usize {
        self.dish
            .iter()
            .enumerate()
            .map(|(n, l)| {
                let (rocks, _, _) = rock_n_stone(l);
                rocks.len() * (self.dish.len() - n)
            })
            .sum()
    }

    fn compare_original(&self) -> bool {
        for (inner1, inner2) in self.original.iter().zip(self.dish.iter()) {
            if inner1.len() != inner2.len() {
                return false;
            }
            if inner1.iter().zip(inner2.iter()).any(|(a, b)| a != b) {
                return false;
            }
        }
        return true;
    }
}

fn rock_n_stone(line: &Vec<char>) -> (Vec<usize>, Vec<usize>, Vec<usize>) {
    let mut rocks = vec![];
    let mut stones = vec![];
    let mut spaces = vec![];
    line.iter().enumerate().for_each(|(n, s)| match s {
        'O' => rocks.push(n),
        '#' => stones.push(n),
        '.' => spaces.push(n),
        _ => unreachable!(),
    });
    (rocks, stones, spaces)
}
