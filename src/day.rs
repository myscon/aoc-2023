use crate::prelude::*;

pub trait Answers {
    fn new(input: PathBuf) -> Self;
    fn read(&self) -> BufReader<File>;
    fn part_one(&self) -> Result<String, Box<dyn Error>>;
    fn part_two(&self) -> Result<String, Box<dyn Error>>;
}

pub struct Day {
    pub input: PathBuf,
}
