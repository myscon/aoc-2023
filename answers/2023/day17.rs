//! YEAR:   2023
//! DAY:    17

use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

use crate::prelude::*;

impl Answers for Day {
    fn part_one(&mut self) -> Result<String, Box<dyn Error>> {
        let mut city = City::new(&self.input);
        city.find_path(Coords { x: 0, y: 0 }, 1, 3);
        Ok(city.get_min_loss().to_string())
    }

    fn part_two(&mut self) -> Result<String, Box<dyn Error>> {
        let mut city = City::new(&self.input);
        city.find_path(Coords { x: 0, y: 0 }, 4, 10);
        Ok(city.get_min_loss().to_string())
    }
}

lazy_static! {
    static ref DIRECTIONS: Vec<Coords> = vec![
        Coords { x: 0, y: 1 },
        Coords { x: 1, y: 0 },
        Coords { x: 0, y: -1 },
        Coords { x: -1, y: 0 },
    ];
}

struct City {
    heatmap: Vec<Vec<u32>>,
    dijkstra: Vec<Vec<u32>>,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Coords {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Visit {
    node: Coords,
    moves: Coords,
    back: Coords,
    loss: u32,
}

impl Ord for Visit {
    fn cmp(&self, other: &Self) -> Ordering {
        other.loss.cmp(&self.loss)
    }
}

impl PartialOrd for Visit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl City {
    fn new(input: &str) -> Self {
        let heatmap = input
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
            .collect_vec();
        let heatman_l = heatmap.len();
        let heatmap_w = heatmap[0].len();
        City {
            heatmap: heatmap,
            dijkstra: vec![vec![u32::MAX; heatmap_w]; heatman_l],
        }
    }

    fn find_path(&mut self, start_node: Coords, min: i32, max: i32) {
        let mut heap = BinaryHeap::new();
        let mut memo = HashSet::new();
        heap.push(Visit {
            node: start_node,
            moves: Coords { x: 0, y: 0 },
            back: Coords { x: 0, y: 0 },
            loss: 0,
        });
        self.dijkstra[start_node.y as usize][start_node.x as usize] = 0;
        while !heap.is_empty() {
            let visit = heap.pop().unwrap();
            memo.insert((visit.node, visit.moves));
            heap.retain(|&v| {
                v.node != visit.node || v.moves.x != visit.moves.x || v.moves.y != visit.moves.y
            });
            for mv in DIRECTIONS.iter() {
                if *mv == visit.back {
                    continue;
                }
                let next = Coords {
                    x: visit.node.x + mv.x,
                    y: visit.node.y + mv.y,
                };
                let mut next_mvs = mv.clone();
                if next_mvs.y != 0 && visit.moves.y != 0 {
                    next_mvs.y += visit.moves.y;
                }
                if next_mvs.x != 0 && visit.moves.x != 0 {
                    next_mvs.x += visit.moves.x;
                }
                if next.x < self.heatmap[0].len() as i32
                    && next.y < self.heatmap.len() as i32
                    && next.x >= 0
                    && next.y >= 0
                    && memo.get(&(next, next_mvs)).is_none()
                    && next_mvs.x.abs() < max + 1
                    && next_mvs.y.abs() < max + 1
                    // I feel like there is a better way of doing this
                    && (((visit.moves.x != 0 && visit.moves.x.abs() < min) ^ (next_mvs.y != 0)) || next_mvs.x != 0)
                    && (((visit.moves.y != 0 && visit.moves.y.abs() < min) ^ (next_mvs.x != 0)) || next_mvs.y != 0)
                {
                    let next_loss = visit.loss + self.heatmap[next.y as usize][next.x as usize];
                    if next_loss < self.dijkstra[next.y as usize][next.x as usize] {
                        if !(next.y as usize == self.heatmap.len() - 1
                            && next.x as usize == self.heatmap[0].len() - 1
                            && next_mvs.x.abs() + next_mvs.y.abs() < min)
                        {
                            self.dijkstra[next.y as usize][next.x as usize] = next_loss;
                        }
                    }
                    heap.push(Visit {
                        node: next,
                        moves: next_mvs,
                        back: Coords { x: -mv.x, y: -mv.y },
                        loss: next_loss,
                    });
                }
            }
        }
    }

    fn get_min_loss(&self) -> u32 {
        self.dijkstra[self.dijkstra.len() - 1][self.dijkstra.len() - 1]
    }
}
