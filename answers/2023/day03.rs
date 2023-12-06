//! YEAR:   2023
//! DAY:    03

use crate::prelude::*;
use std::{collections::VecDeque, io::BufRead};
use fancy_regex::Regex;

lazy_static! {
    static ref SYMBOL: Regex = Regex::new(r"[^\d.]").unwrap();
    static ref NUMBER: Regex = Regex::new(r"\d+").unwrap();
    static ref GEAR: Regex = Regex::new(r"\*").unwrap();
}

impl Answers for Day {
    fn new(input: PathBuf) -> Self {
        Day { input }
    }

    fn read(&self) -> BufReader<File> {
        let file = File::open(self.input.to_owned()).unwrap();
        return BufReader::new(file);
    }

    fn part_one(&self) -> Result<String, Box<dyn Error>> {
        let mut reader = self.read();
        let mut aggreg = 0;
        let mut targets = VecDeque::new();
        targets.push_back("".to_string());
        for _i in 0..2 {
            enqueue_line(&mut targets, &mut reader);
        }
        let mut num_matches = NUMBER.find_iter(&targets[1]);
        while !targets[1].is_empty() {
            for num in &mut num_matches {
                let num = num.unwrap();
                let start = num.start().saturating_sub(1);
                let end = num.end().saturating_add(1).min(140);
                if symbol_search(&targets, start, end) {
                    aggreg += num.as_str().parse::<i32>().unwrap()
                }
            }
            let _ = targets.pop_front();
            enqueue_line(&mut targets, &mut reader);
            num_matches = NUMBER.find_iter(&targets[1]);
        }
        Ok(aggreg.to_string())
    }

    fn part_two(&self) -> Result<String, Box<dyn Error>> {
        let mut reader = self.read();
        let mut aggreg = 0;
        let mut targets = VecDeque::new();
        targets.push_back("".to_string());
        for _i in 0..2 {
            enqueue_line(&mut targets, &mut reader);
        }
        let mut num_matches = GEAR.find_iter(&targets[1]);
        while !targets[1].is_empty() {
            for num in &mut num_matches {
                let num = num.unwrap();
                let start = num.start().saturating_sub(3);
                let end = num.end().saturating_add(3).min(140);
                aggreg += number_search(&targets, start, end);
            }
            let _ = targets.pop_front();
            enqueue_line(&mut targets, &mut reader);
            num_matches = GEAR.find_iter(&targets[1]);
        }
        Ok(aggreg.to_string())
    }
}

fn number_search(targets: &VecDeque<String>, start: usize, end: usize) -> i32 {
    let mut nums = vec!();
    for target in targets {
        if !target.is_empty() {
            let num_matches = NUMBER.find_iter(&target[start..end]);
            for num_match in num_matches {
                let num_res = num_match.unwrap();
                let st = num_res.start();
                let ed = num_res.end();
                if st < 5 && ed > 2 {
                    nums.push(num_res.as_str())
                }
            }
        }
    }
    if nums.len() == 2{
        return nums[0].parse::<i32>().unwrap()*nums[1].parse::<i32>().unwrap()
    } else {
        return 0;
    }
}

fn symbol_search(targets: &VecDeque<String>, start: usize, end: usize) -> bool {
    for target in targets {
        if !target.is_empty() && SYMBOL.find(&target[start..end]).unwrap().is_some() {
            return true;
        }
    }
    return false;
}

fn enqueue_line(queue: &mut VecDeque<String>, reader: &mut BufReader<File>) {
    let mut texts = String::new();
    let _ = reader.read_line(&mut texts);
    queue.push_back(texts);
}
