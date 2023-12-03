//! YEAR: {{2023}}
//! DAY:  {{02}}

use crate::prelude::*;
use fancy_regex::Regex;
use std::io::BufRead;

lazy_static! {
    static ref RED: i32 = 12;
    static ref GREEN: i32 = 13;
    static ref BLUE: i32 = 14;
    static ref GAME_REG: Regex = Regex::new(r"(?<=Game )\d+").unwrap();
    static ref COLOR_REG: Regex = Regex::new(r"(\d+) (red|green|blue)").unwrap();
}

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
            let game_match = GAME_REG.find(&read_line).unwrap().unwrap();
            let game_number = game_match.as_str().parse::<i32>().unwrap();
            let color_matches = COLOR_REG.captures_iter(&read_line);
            let mut valid_game = true;
            for captures in color_matches {
                let captures = captures?;
                let value = captures
                    .get(1)
                    .map(|m| m.as_str().parse::<i32>().unwrap())
                    .unwrap();
                let color = captures.get(2).map(|m| m.as_str()).unwrap();
                let max_value = match color {
                    "red" => *RED,
                    "green" => *GREEN,
                    "blue" => *BLUE,
                    _ => panic!("You might be color blind."),
                };
                if value > max_value {
                    valid_game = false;
                    break;
                }
            }
            if valid_game {
                aggreg += game_number;
            }
        }
        Ok(aggreg.to_string())
    }

    fn part_two(&self) -> Result<String, Box<dyn Error>> {
        let reader = self.read();
        let mut aggreg = 0;
        for line in reader.lines() {
            let read_line = line.unwrap();
            let color_matches = COLOR_REG.captures_iter(&read_line);
            let mut red_max = 0;
            let mut green_max = 0;
            let mut blue_max = 0;
            for captures in color_matches {
                let captures = captures?;
                let value = captures
                    .get(1)
                    .map(|m| m.as_str().parse::<i32>().unwrap())
                    .unwrap();
                let color = captures.get(2).map(|m| m.as_str()).unwrap();
                match color {
                    "red" | "green" | "blue" => {
                        let max_value = match color {
                            "red" => &mut red_max,
                            "green" => &mut green_max,
                            "blue" => &mut blue_max,
                            _ => unreachable!(),
                        };

                        if value > *max_value {
                            *max_value = value;
                        }
                    }
                    _ => panic!("You might be color blind."),
                }
            }
            aggreg += red_max * green_max * blue_max;
        }
        Ok(aggreg.to_string())
    }
}
