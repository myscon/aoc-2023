//! YEAR:   2023
//! DAY:    05

use crate::prelude::*;
use crate::regex::regex;
use rayon::iter::ParallelBridge;
use rayon::prelude::ParallelIterator;

impl Answers for Day {
    fn part_one(&mut self) -> String {
        let mut seeds_str = String::new();
        let _ = self.reader.read_line(&mut seeds_str);
        // I should probably start writing srfs so I don't have to keep copying and pasting
        let mut seeds = regex("number")
            .find_iter(&seeds_str)
            .map(|s| s.unwrap().as_str().parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let mut check = vec![true; seeds.len()];
        let mut read_line = String::new();
        let _ = self.reader.read_line(&mut read_line);
        while read_line.len() > 0 {
            if !read_line.is_empty() {
                if let Some(first_char) = read_line.chars().next() {
                    if !first_char.is_numeric() {
                        continue;
                    };
                }
                let alma_values = read_line
                    .split(" ")
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<usize>>();
                let dest_range = alma_values[1] + alma_values[2];
                println!("{}", alma_values[2]);
                for i in 0..seeds.len() {
                    if check[i] && seeds[i] >= alma_values[1] && seeds[i] < dest_range {
                        seeds[i] = seeds[i] - alma_values[1] + alma_values[0];
                        check[i] = false;
                    }
                }
            } else {
                check.iter_mut().for_each(|e| *e = true);
            }
            read_line.clear();
            let _ = self.reader.read_line(&mut read_line);
        }
        seeds.iter().min().unwrap().to_string()
    }

    fn part_two(&mut self) -> String {
        let mut seeds_str = String::new();
        let _ = self.reader.read_line(&mut seeds_str);
        let seeds = regex("number")
            .find_iter(&seeds_str)
            .map(|s| s.unwrap().as_str().parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let mut lines = vec![];
        parse_lines(&mut self.reader, &mut lines);
        let mut seed_bank = vec![];
        let mut answer = usize::MAX;
        for i in (0..seeds.len()).step_by(2) {
            for j in 0..seeds[i+1] {
                seed_bank.push(seeds[i] + j);
            }
            answer = answer.min(seed_bank
                .iter()
                .par_bridge()
                .map(|s| get_location(*s, &lines))
                .min()
                .unwrap());
            seed_bank.clear();
        }
        answer.to_string()
    }
}

// oonga boonga go loop loop this made me feel like I don't know how to code
// https://www.youtube.com/watch?v=Vw6hcx8jUeY Man this takes me back
fn get_location(seed: usize, lines: &Vec<Vec<usize>>) -> usize {
    let mut target = seed;
    let mut check = true;
    for line in lines {
        if check && line.len() > 0 {
            if check && target >= line[1] && target < line[1] + line[2] {
                target = target - line[1] + line[0];
                check = false
            }
        } else if line.len() == 0 {
            check = true;
        }
    }
    return target;
}

fn parse_lines(reader: &mut BufReader<File>, buffer: &mut Vec<Vec<usize>>) {
    for line in reader.lines() {
        let read_line = line.unwrap();
        if read_line.is_empty() {
            buffer.push(vec![]);
        } else {
            if let Some(first_char) = read_line.chars().next() {
                if first_char.is_numeric() {
                    let alma_values = read_line
                        .split(" ")
                        .map(|s| s.parse().unwrap())
                        .collect::<Vec<usize>>();
                    buffer.push(alma_values);
                } else {
                    continue;
                }
            }
        }
    }
}
