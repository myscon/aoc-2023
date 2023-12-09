//! YEAR:   {{2023}}
//! DAY:    {{09}}

use crate::prelude::*;
use rayon::iter::{IntoParallelIterator, ParallelExtend};
use rayon::prelude::ParallelIterator;

impl Answers for Day {
    fn part_one(&mut self) -> Result<String, Box<dyn Error>> {
        let mut history = History::new(&mut self.reader);
        history
            .histories
            .par_extend(history.lines.into_par_iter().map(|v| parse_line(&v, false)));
        Ok(history
            .histories
            .iter()
            .fold(0, |acc, x| acc + x)
            .to_string())
    }

    fn part_two(&mut self) -> Result<String, Box<dyn Error>> {
        let mut history = History::new(&mut self.reader);
        history
            .histories
            .par_extend(history.lines.into_par_iter().map(|v| parse_line(&v, true)));
        Ok(history
            .histories
            .iter()
            .fold(0, |acc, x| acc + x)
            .to_string())
    }
}

struct History {
    lines: Vec<Vec<i32>>,
    histories: Vec<i32>,
}

trait Predictions {
    fn new(reader: &mut BufReader<File>) -> Self;
}

impl Predictions for History {
    fn new(reader: &mut BufReader<File>) -> Self {
        let mut pred_buffer = String::new();
        let _ = reader.read_line(&mut pred_buffer);
        let mut lines = vec![];
        while pred_buffer.len() > 0 {
            let line_split = pred_buffer
                .trim()
                .split(" ")
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            lines.push(line_split);
            pred_buffer.clear();
            _ = reader.read_line(&mut pred_buffer);
        }
        History {
            lines: lines,
            histories: vec![],
        }
    }
}

fn parse_line(line: &Vec<i32>, reverse: bool) -> i32 {
    let mut curr_line = line.clone();
    let mut next_line = vec![];
    let mut targ_nums = vec![];

    while curr_line.len() > 1 {
        for i in 1..curr_line.len() {
            next_line.push(curr_line[i] - curr_line[i - 1]);
        }
        if reverse {
            targ_nums.push(*curr_line.first().unwrap());
        } else {
            targ_nums.push(*curr_line.last().unwrap());
        }
        curr_line = next_line.clone();
        next_line.clear();
    }
    if reverse {
        return targ_nums.iter().fold(0, |acc, n| -(n + acc));
    } else {
        return targ_nums.iter().sum();   
    }
}
