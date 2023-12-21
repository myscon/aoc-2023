//! YEAR:   2023
//! DAY:    16

use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use crate::prelude::*;

impl Answers for Day {
    fn part_one(&mut self) -> String {
        let mut contraption = Contraption::new(&self.input);
        contraption.traverse_grid(Point { x: 0, y: 0 }, Point { x: 1, y: 0 });
        contraption.tally_grid().to_string()
    }

    fn part_two(&mut self) -> String {
        let mut contraption = Contraption::new(&self.input);
        contraption.config_actions().to_string()
    }
}

struct Contraption {
    grid: Vec<Vec<Tile>>,
    reset: Vec<Vec<Tile>>, // ugh I don't like this. should've made part one better
    mirrors: HashMap<char, i32>,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash)]
struct Tile {
    value: char,
    energy: bool,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Contraption {
    fn new(input: &str) -> Self {
        let grid = input
            .lines()
            .map(|s| {
                s.chars()
                    .map(|c| Tile {
                        value: c,
                        energy: false,
                    })
                    .collect_vec()
            })
            .collect_vec();
        Contraption {
            grid: grid.clone(),
            reset: grid.clone(),
            mirrors: HashMap::from([('/', -1), ('\\', 1)]),
        }
    }

    fn config_actions(&mut self) -> usize {
        let mut max = 0;
        for (y, d) in [(0, 1), (self.grid.len() - 1, -1)] {
            for x in 0..self.grid[0].len() - 1 {
                self.traverse_grid(
                    Point {
                        x: x as i32,
                        y: y as i32,
                    },
                    Point { x: 0 as i32, y: d },
                );
                max = max.max(self.tally_grid());
                self.reset_grid();
            }
        }
        for (x, d) in [(0, 1), (self.grid[0].len() - 1, -1)] {
            for y in 0..self.grid.len() - 1 {
                self.traverse_grid(
                    Point {
                        x: x as i32,
                        y: y as i32,
                    },
                    Point { x: d, y: 0 as i32 },
                );
                max = max.max(self.tally_grid());
                self.reset_grid();
            }
        }
        max
    }

    fn traverse_grid(&mut self, start: Point, direction: Point) {
        let mut action_stack = &mut vec![];
        let mut memo = HashSet::new();
        self.push_action(&mut action_stack, start, direction);
        while !action_stack.is_empty() {
            let (loc, dir) = action_stack.pop().unwrap();
            if memo.get(&(loc, dir)).is_none() {
                memo.insert((loc, dir));
                self.grid[loc.y as usize][loc.x as usize].energy = true;
                let next_loc = Point {
                    x: loc.x + dir.x,
                    y: loc.y + dir.y,
                };
                if next_loc.y < self.grid.len() as i32
                    && next_loc.y >= 0
                    && next_loc.x < self.grid[0].len() as i32
                    && next_loc.x >= 0
                {
                    self.push_action(action_stack, next_loc, dir)
                }
            }
        }
    }

    fn push_action(&self, action_stack: &mut Vec<(Point, Point)>, next_loc: Point, dir: Point) {
        let next_dir = &self.grid[next_loc.y as usize][next_loc.x as usize];
        match next_dir.value {
            '.' => {
                action_stack.push((next_loc, dir));
            }
            '/' | '\\' => {
                let reflect = self.mirrors.get(&next_dir.value).unwrap();
                if dir.x != 0 {
                    action_stack.push((
                        next_loc,
                        Point {
                            x: 0,
                            y: reflect * dir.x,
                        },
                    ))
                }
                if dir.y != 0 {
                    action_stack.push((
                        next_loc,
                        Point {
                            x: reflect * dir.y,
                            y: 0,
                        },
                    ))
                }
            }
            '|' => {
                if dir.x != 0 {
                    [1, -1]
                        .iter()
                        .for_each(|&d| action_stack.push((next_loc.clone(), Point { x: 0, y: d })))
                } else {
                    action_stack.push((next_loc, dir));
                }
            }
            '-' => {
                if dir.y != 0 {
                    [1, -1]
                        .iter()
                        .for_each(|&d| action_stack.push((next_loc.clone(), Point { x: d, y: 0 })))
                } else {
                    action_stack.push((next_loc, dir));
                }
            }
            _ => unreachable!(),
        }
    }

    fn reset_grid(&mut self) {
        self.grid = self.reset.clone();
    }

    fn tally_grid(&self) -> usize {
        self.grid
            .iter()
            .map(|l| {
                l.iter()
                    .map(|c| if c.energy { 1 } else { 0 })
                    .sum::<usize>()
            })
            .sum()
    }
}
