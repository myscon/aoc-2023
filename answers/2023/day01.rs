//! YEAR:   2023
//! DAY:    01

use crate::prelude::*;
use crate::regex::regex;

impl Answers for Day {
    fn part_one(&mut self) -> Result<String, Box<dyn Error>> {
        let mut aggreg = 0;
        let mut read_line = String::new();
        let _ = self.reader.read_line(&mut read_line);
        while read_line.len() > 0 {
            let mut clear = true;
            let mut first = 0;
            let mut last = 0;
            for ch in read_line.chars() {
                if let Some(num) = ch.to_digit(10) {
                    last = num;
                    if clear {
                        clear = false;
                        first = num;
                    }
                }
            }
            read_line.clear();
            let _ = self.reader.read_line(&mut read_line);
            aggreg += first * 10 + last;
        }
        Ok(aggreg.to_string())
    }

    fn part_two(&mut self) -> Result<String, Box<dyn Error>> {
        let mut aggreg = 0;
        let mut read_line = String::new();
        let _ = self.reader.read_line(&mut read_line);
        while read_line.len() > 0 {
            let rev = read_line.chars().rev().collect::<String>();
            let first = unwrap_present(false, regex("num_word_f").find(&read_line));
            let last = unwrap_present(true, regex("num_word_r").find(&rev));
            aggreg += first * 10 + last;
            read_line.clear();
            let _ = self.reader.read_line(&mut read_line);
        }
        Ok(aggreg.to_string())
    }
}

fn unwrap_present(
    rev: bool,
    present: Result<Option<fancy_regex::Match<'_>>, fancy_regex::Error>,
) -> i32 {
    let unwrapped_present = present.unwrap().unwrap().as_str();
    if rev {
        return play_with_present(unwrapped_present.chars().rev().collect::<String>().as_str());
    } else {
        return play_with_present(unwrapped_present);
    }
}

fn play_with_present(present: &str) -> i32 {
    match present {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => present.parse().unwrap(),
    }
}
