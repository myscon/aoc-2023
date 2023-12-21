use crate::prelude::*;

pub struct Day {
    pub input: String,
    pub reader: BufReader<fs::File>,
}

impl Day {
    pub fn new(input: &PathBuf) -> Self {
        Day {
            input: fs::read_to_string(input).expect("File read error"),
            // I started the aoc using bufRead and changed to read_string. I didn't want to change
            // previous days so I'm leaving this here.
            reader: BufReader::new(fs::File::open(input).expect("File open error")),
        }
    }
}

pub trait Answers {
    fn part_one(&mut self) -> String;
    fn part_two(&mut self) -> String;
}
