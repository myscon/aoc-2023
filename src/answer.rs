//! YEAR:   2023
//! DAY:    04

use crate::prelude::*;
use std::{collections::VecDeque, io::BufRead};

impl Answers for Day {
    fn new(input: PathBuf) -> Self {
        Day { input }
    }

    fn read(&self) -> BufReader<File> {
        let file = File::open(self.input.to_owned()).unwrap();
        return BufReader::new(file);
    }

    fn part_one(&self) -> Result<String, Box<dyn Error>> {
        let reader = self.read();
        let mut aggreg = 0;
        let base: i32 = 2;
        for line in reader.lines() {
            let read_line = line.unwrap();
            // I refuse to iterate char by char its too late at night
            let card_split = read_line.split(": ").collect::<Vec<&str>>()[1];
            let num_split = card_split.split("|").collect::<Vec<&str>>();
            let win_split = num_split[0].split_whitespace().collect::<Vec<&str>>();
            let lot_split = num_split[1].split_whitespace().collect::<Vec<&str>>();
            let mut count = 0;
            for num in lot_split {
                if win_split.contains(&num) {
                    count += 1;
                }
            }
            let card_score;
            if count > 0 {
                card_score = 1 * base.pow(count - 1);
            } else {
                card_score = 0;
            }
            aggreg += card_score
        }
        Ok(aggreg.to_string())
    }

    fn part_two(&self) -> Result<String, Box<dyn Error>> {
        let reader = self.read();
        let mut aggreg = 0;
        let mut card_acc = VecDeque::new();
        for line in reader.lines() {
            let read_line = line.unwrap();
            // Maybe I should have made something better...
            let card_split = read_line.split(": ").collect::<Vec<&str>>()[1];
            let num_split = card_split.split("|").collect::<Vec<&str>>();
            let win_split = num_split[0].split_whitespace().collect::<Vec<&str>>();
            let lot_split = num_split[1].split_whitespace().collect::<Vec<&str>>();
            let mut count = 0;
            for num in lot_split {
                if win_split.contains(&num) {
                    count += 1;
                }
            }
            let copies = card_acc.pop_front().unwrap_or(0);
            let base = 1 + copies;
            // please don't laugh at me...
            if count > 0 {
                if card_acc.len() == 0 {
                    for _i in 0..count {
                        card_acc.push_back(base)
                    }
                } else if card_acc.len() >= count {
                    let mut i = 0;
                    // so VecDeque apparently doesn't support random access and indexing is O(n)
                    for card in card_acc.iter_mut() {
                        *card += base;
                        i += 1;
                        if i == count {
                            break;
                        }
                    }
                } else if card_acc.len() < count {
                    for card in card_acc.iter_mut() {
                        *card += base;
                    }
                    for _i in 0..count - card_acc.len() {
                        card_acc.push_back(base);
                    }
                }
            }
            aggreg += base;
        }
        Ok(aggreg.to_string())
    }
}
