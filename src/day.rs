use crate::prelude::*;

pub struct Day {
    pub input: String,
}

impl Day {
    pub fn new(input: PathBuf) -> Self {
        Day { input: fs::read_to_string(input).expect("File read error") }
    }
}

pub trait Answers {
    fn part_one(&mut self) -> String;
    fn part_two(&mut self) -> String;
}