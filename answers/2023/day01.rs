//! YEAR:   2023
//! DAY:    01

use crate::{Answers, Day};
use fancy_regex::Regex;
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

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
        for line in reader.lines() {
            let read_line = line.unwrap();
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
            aggreg += first * 10 + last;
        }
        Ok(aggreg.to_string())
    }

    fn part_two(&self) -> Result<String, Box<dyn Error>> {
        let reader = self.read();
        let for_pat = r"(one|two|three|four|five|six|seven|eight|nine|\d)";
        let rev_pat = r"(eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|\d)";
        let pattern_f = Regex::new(for_pat).unwrap();
        let pattern_l = Regex::new(rev_pat).unwrap();
        // It turns out this doesn't cover overlapping values
        // let pattern_l = Regex::new(&format!(r"({})(?!.*\b\1\b)", for_pat)).unwrap();

        let mut aggreg = 0;
        for line in reader.lines() {
            let read_line = line.unwrap();
            let rev = read_line.chars().rev().collect::<String>();
            let first = unwrap_present(false, pattern_f.find(&read_line));
            let last = unwrap_present(true, pattern_l.find(&rev));
            aggreg += first * 10 + last;
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
