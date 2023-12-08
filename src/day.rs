use crate::prelude::*;

pub trait Answers {
    fn part_one(&mut self) -> Result<String, Box<dyn Error>>;
    fn part_two(&mut self) -> Result<String, Box<dyn Error>>;
}

pub struct Day {
    pub input: PathBuf,
    pub reader: BufReader<File>
}

pub trait Constructor {
    fn new(input: PathBuf) -> Self;
}

impl Constructor for Day {
    fn new(input: PathBuf) -> Self {
        let file = File::open(input.to_owned()).unwrap();
        Day { input, reader: BufReader::new(file) }
    }
}