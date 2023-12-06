//! YEAR:   2023
//! DAY:    06

use crate::prelude::*;
use std::{io::BufRead, usize};

use crate::regex;

impl Answers for Day {
    fn new(input: PathBuf) -> Self {
        Day { input }
    }

    fn reader(&self) -> BufReader<File> {
        let file = File::open(self.input.to_owned()).unwrap();
        return BufReader::new(file);
    }

    fn part_one(&self) -> Result<String, Box<dyn Error>> {
        let mut reader = self.reader();
        let times = get_numbers(&mut reader);
        let distances = get_numbers(&mut reader);
        let mut count_agg = vec![];
        for i in 0..times.len() {
            count_agg.push(get_winners(times[i], distances[i]))
        }
        Ok(count_agg.iter().fold(1, |acc, &x| acc * x).to_string())
    }

    fn part_two(&self) -> Result<String, Box<dyn Error>> {
        let mut reader = self.reader();
        let time = get_number(&mut reader);
        let distance = get_number(&mut reader);
        let loop_range = 0..time;
        let count_agg = loop_range
        .map(|s| get_winner(time, s, distance))
        .fold(0, |acc, x| acc + x);
        Ok(count_agg.to_string())
    }
}

fn get_winner(time: usize, speed: usize, distance: usize) -> usize {
    let remainder = time - speed;
    let travel = speed * remainder;
    if travel > distance {
        return 1;
    } else {
        return 0;
    }
}

fn get_winners(time: usize, distance: usize) -> usize {
    let mut count = 0;
    for j in 0..time {
        count += get_winner(time, j, distance);
    }
    return count;
}

fn get_number(reader: &mut BufReader<File>) -> usize {
    let mut buffer = String::new();
    let _ = reader.read_line(&mut buffer);
    let num_match = regex::NUMBER
        .find_iter(&buffer)
        .map(|f| f.unwrap().as_str())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    return num_match;
}

fn get_numbers(reader: &mut BufReader<File>) -> Vec<usize> {
    let mut buffer = String::new();
    let _ = reader.read_line(&mut buffer);
    let num_match = regex::NUMBER
        .find_iter(&buffer)
        .map(|f| f.unwrap().as_str().parse::<usize>().unwrap())
        .collect();
    return num_match;
}
