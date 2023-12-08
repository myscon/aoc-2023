//! YEAR:   2023
//! DAY:    06

use crate::prelude::*;
use crate::regex::regex;

impl Answers for Day {
    fn part_one(&mut self) -> Result<String, Box<dyn Error>> {
        let times = self.get_numbers();
        let distances = self.get_numbers();
        let mut count_agg = vec![];
        for i in 0..times.len() {
            count_agg.push(get_winners(times[i], distances[i]))
        }
        Ok(count_agg.iter().fold(1, |acc, &x| acc * x).to_string())
    }

    fn part_two(&mut self) -> Result<String, Box<dyn Error>> {
        let time = self.get_number();
        let distance = self.get_number();
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

trait Numbers {
    fn get_number(&mut self) -> usize;
    fn get_numbers(&mut self) -> Vec<usize>;
}

impl Numbers for Day {
    fn get_number(&mut self) -> usize {
        let mut buffer = String::new();
        let _ = &mut self.reader.read_line(&mut buffer);
        let num_match = regex("number")
            .find_iter(&buffer)
            .map(|f| f.unwrap().as_str())
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        return num_match;
    }
    
    fn get_numbers(&mut self) -> Vec<usize> {
        let mut buffer = String::new();
        let _ = self.reader.read_line(&mut buffer);
        let num_match = regex("number")
            .find_iter(&buffer)
            .map(|f| f.unwrap().as_str().parse::<usize>().unwrap())
            .collect();
        return num_match;
    }
}
