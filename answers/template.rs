use crate::prelude::*;

impl Answers for Day {
    fn part_one(&mut self) -> Result<String, Box<dyn Error>>  {
        // Answer logic here
        Ok("".into())
    }

    fn part_two(&mut self) -> Result<String, Box<dyn Error>> {
        // Answer logic here
        Ok("".into())
    }
}

struct Advent {
}

trait Code {
    fn new(input: &str) -> Self;
}

impl Code for Advent {
    fn new(input: &str) -> Self {
        Advent { }
    }
}