use crate::prelude::*;

pub trait Answers {
    fn part_one(&mut self) -> Result<String, Box<dyn Error>>;
    fn part_two(&mut self) -> Result<String, Box<dyn Error>>;
}

pub struct Day {
    pub input: String,
}

pub trait Constructor {
    fn new(input: PathBuf) -> Self;
}

impl Constructor for Day {
    fn new(input: PathBuf) -> Self {
        Day { input: fs::read_to_string(input).expect("File read error") }
    }
}