//! YEAR:   2023
//! DAY:    08

use crate::prelude::*;
use crate::regex::regex;
use std::collections::HashMap;
use std::sync::mpsc::{self, channel, sync_channel};
use std::sync::{Arc, Barrier, Mutex};
use std::thread;
use num_integer::lcm;

impl Answers for Day {
    fn part_one(&mut self) -> String {
        let mut desert = Desert::new(&mut self.reader, false);
        desert.traverse();
        desert.steps.to_string()
    }

    fn part_two(&mut self) -> String {
        let mut desert = Desert::new(&mut self.reader, true);
        desert.spooky_traverse();
        desert.steps.to_string()
    }
}

struct Desert {
    directions: Vec<usize>,
    current: String,
    spooky_current: Vec<String>,
    steps: usize,
    // I want to know if &str is much better here because I don't want to deal with moving
    // ownership just to do this. This is just more convenient
    desert_map: HashMap<String, Vec<String>>,
}

trait Directions {
    fn new(reader: &mut BufReader<File>, spooky: bool) -> Self;
    fn traverse(&mut self);
    fn spooky_traverse(&mut self);
}

impl Directions for Desert {
    fn new(reader: &mut BufReader<File>, spooky: bool) -> Self {
        let mut dir_buffer = String::new();
        let _ = reader.read_line(&mut dir_buffer);

        let mut dir_map = HashMap::new();
        let mut spooky_current = vec![];
        let directions = dir_buffer
            .trim()
            .chars()
            .map(|s| match s {
                'L' => 0, 
                'R' => 1,
                _ => unreachable!()
            })
            .collect::<Vec<usize>>();

        for _ in 0..2 {
            dir_buffer.clear();
            _ = reader.read_line(&mut dir_buffer);
        }

        let matcher = regex("map_node");
        while dir_buffer.len() > 0 {
            let fork = matcher
                .find_iter(&dir_buffer)
                .map(|s| s.unwrap().as_str())
                .collect::<Vec<&str>>();
            dir_map.insert(
                fork[0].to_string(),
                vec![fork[1].to_string(), fork[2].to_string()],
            );
            if spooky && fork[0].chars().last().unwrap() == 'A' {
                spooky_current.push(fork[0].to_string());
            }
            dir_buffer.clear();
            _ = reader.read_line(&mut dir_buffer);
        }
        Desert {
            directions: directions,
            current: "AAA".to_string(),
            spooky_current: spooky_current,
            steps: 0,
            desert_map: dir_map,
        }
    }

    fn traverse(&mut self) {
        let mut steps = 0;
        let direc_num = self.directions.len();
        
        while self.current != "ZZZ" {
            let direction = self.directions[steps % direc_num];
            self.current = self.desert_map.get(&self.current).unwrap()[direction].clone();
            steps += 1;
            if self.current == "ZZZ" {
                self.steps = steps;
            }
        }
        
    }

    // I'm ashamed to say the threads are for when I tried to brute force it and just 
    // iterate through all the directions. While I was waiting for it to complete I entered
    // u32::MAX into the answer sheet to see how much more I should wait and it was too small. 
    // I didn't feel like deleting it so here we are.
    fn spooky_traverse(&mut self) {
        let ghost_num = self.spooky_current.len();
        let direc_num = self.directions.len();
        let spooky_flag = Arc::new(Mutex::new(false));
        let (sender, receiver) = sync_channel(u16::MAX as usize);
        for i in 0..ghost_num {
            let (spooky, sender) = (Arc::clone(&spooky_flag), sender.clone(),
            );
            let directions = self.directions.clone();
            let desert_map = self.desert_map.clone();
            let ghost = self.spooky_current[i].clone();
            thread::spawn(move || {
                let mut location = ghost.clone();
                let mut steps = 0;
                while !*spooky.lock().unwrap() {
                    let turn = directions[steps % direc_num];
                    location = desert_map.get(&location).unwrap()[turn].clone();
                    steps += 1;
                    if location.chars().last().unwrap() == 'Z' {
                        // if this blocks the ghosts shouldn't really be producing more anyway
                        sender.send((ghost.clone(), steps)).unwrap();
                    }
                }
            });
        }
        let receiver = Arc::new(Mutex::new(receiver));
        let spooky = Arc::clone(&spooky_flag);
        let (tx, rx) = channel();
        thread::spawn(move || {
            let mut ghost_map = HashMap::new();
            '_spooky_check: while !*spooky.lock().unwrap() {
                if let Ok(rx) = receiver.lock() {
                    match rx.try_recv() {
                        Ok(data) => {
                            let _ = ghost_map.insert(data.0, data.1);
                            if ghost_map.values().len() == ghost_num {
                                *spooky.lock().unwrap() = true;
                                let steps = ghost_map.values().fold(1, |acc, &x| lcm(acc, x));
                                tx.send(steps).unwrap();
                            }
                        }
                        Err(mpsc::TryRecvError::Empty) => {
                            thread::sleep(std::time::Duration::from_millis(50));
                        }
                        Err(mpsc::TryRecvError::Disconnected) => {
                            break;
                        }
                    }
                }
            }
        });
        self.steps = rx.recv().unwrap();
    }
}