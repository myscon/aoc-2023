//! YEAR:   2023
//! DAY:    15

use crate::prelude::*;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator};
use rayon::prelude::ParallelIterator;

impl Answers for Day {
    fn part_one(&mut self) -> Result<String, Box<dyn Error>> {
        let mut reflector = Reflector::new(&self.input);
        reflector.verify_hashes();
        Ok(reflector.verify_hashes().to_string())
    }

    fn part_two(&mut self) -> Result<String, Box<dyn Error>> {
        let mut reflector = Reflector::new(&self.input);
        reflector.hash_labels();
        reflector.install_lenses();
        Ok(reflector.calculate_power().to_string())
    }
}

struct Reflector<'a> {
    sequence: Vec<&'a [u8]>,
    hashes: Option<Vec<usize>>,
    boxes: Option<Vec<Vec<(&'a [u8], u8)>>>,
}

impl<'a> Reflector<'a> {
    fn new(input: &'a str) -> Self {
        let sequence = input
            .trim()
            .split(',')
            .map(|s| s.as_bytes())
            .collect::<Vec<&[u8]>>();
        Reflector {
            sequence: sequence.clone(),
            hashes: None,
            boxes: None,
        }
    }

    fn calculate_power(&self) -> usize {
        let boxes = self.boxes.as_ref().unwrap();
        let mut acc = 0;
        for bnum in 0..boxes.len() {
            for lnum in 0..boxes[bnum].len() {
                acc += (1 + bnum) * (1 + lnum) * (boxes[bnum][lnum].1 - b'0') as usize
            }
        }
        acc
    }

    fn install_lenses(&mut self) {
        self.boxes = Some(vec![vec![]; 256]);
        let boxes = self.boxes.as_mut().unwrap();
        let hashes = self.hashes.as_ref().unwrap();

        for i in 0..self.sequence.len() {
            match self.sequence[i].last().unwrap() {
                b'-' => {
                    boxes[hashes[i]].retain(|&(u, _)| {
                        !u.starts_with(&self.sequence[i][..self.sequence[i].len() - 1])
                    });
                }
                _ => {
                    let insert = (
                        &self.sequence[i][..self.sequence[i].len() - 1],
                        self.sequence[i][self.sequence[i].len() - 1],
                    );
                    if let Some(index) = boxes[hashes[i]].iter().position(|&(u, _)| {
                        u.starts_with(&self.sequence[i][..self.sequence[i].len() - 1])
                    }) {
                        boxes[hashes[i]][index] = insert
                    } else {
                        boxes[hashes[i]].push(insert);
                    }
                }
            }
        }
    }

    fn hash_labels(&mut self) {
        self.hashes = Some(vec![]);
        self.sequence
            .par_iter()
            .map(|a| hash_value(a, true))
            .collect_into_vec(self.hashes.as_mut().unwrap())
    }

    fn verify_hashes(&mut self) -> usize {
        self.sequence
            .par_iter()
            .map(|a| hash_value(a, false))
            .sum::<usize>()
    }
}

fn hash_value(input: &[u8], label_only: bool) -> usize {
    return input.iter().fold(0, |acc, &b| {
        if label_only && b < 62 {
            acc
        } else {
            (acc + b as usize) * 17 % 256
        }
    });
}
