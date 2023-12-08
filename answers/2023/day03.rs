//! YEAR:   2023
//! DAY:    03

use crate::prelude::*;
use crate::regex:: {NUMBER, GEAR};
use std::collections::VecDeque;

impl Answers for Day {
    fn part_one(&mut self) -> Result<String, Box<dyn Error>> {
        let mut aggreg = 0;
        let mut targets = VecDeque::new();
        targets.push_back("".to_string());
        for _i in 0..2 {
            self.enqueue_line(&mut targets);
        }
        let mut num_matches = regex("number").find_iter(&targets[1]);
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
            self.enqueue_line(&mut targets);
            num_matches = regex("number").find_iter(&targets[1]);
        }
        Ok(aggreg.to_string())
    }

    fn part_two(&mut self) -> Result<String, Box<dyn Error>> {
        let mut aggreg = 0;
        let mut targets = VecDeque::new();
        targets.push_back("".to_string());
        for _i in 0..2 {
            self.enqueue_line(&mut targets);
        }
        let mut num_matches = regex("asterisk").find_iter(&targets[1]);
        while !targets[1].is_empty() {
            for num in &mut num_matches {
                let num = num.unwrap();
                let start = num.start().saturating_sub(3);
                let end = num.end().saturating_add(3).min(140);
                aggreg += number_search(&targets, start, end);
            }
            let _ = targets.pop_front();
            self.enqueue_line(&mut targets);
            num_matches = regex("asterisk").find_iter(&targets[1]);
        }
        Ok(aggreg.to_string())
    }
}

fn number_search(targets: &VecDeque<String>, start: usize, end: usize) -> i32 {
    let mut nums = vec!();
    for target in targets {
        if !target.is_empty() {
            let num_matches = regex("number").find_iter(&target[start..end]);
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
        if !target.is_empty() && regex("non_digit").find(&target[start..end]).unwrap().is_some() {
            return true;
        }
    }
    return false;
}

trait Queue {
    fn enqueue_line(&mut self, queue: &mut VecDeque<String>);
}

impl Queue for Day {
    fn enqueue_line(&mut self, queue: &mut VecDeque<String>,) {
        let mut texts = String::new();
        let _ = self.reader.read_line(&mut texts);
        queue.push_back(texts);
    }   
}
