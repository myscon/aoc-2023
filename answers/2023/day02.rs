//! YEAR:   2023
//! DAY:    02

use crate::prelude::*;
use crate::regex::regex;

lazy_static! {
    static ref RED: i32 = 12;
    static ref GREEN: i32 = 13;
    static ref BLUE: i32 = 14;
}

impl Answers for Day {
    fn part_one(&mut self) -> Result<String, Box<dyn Error>> {
        let mut aggreg = 0;
        let mut read_line = String::new();
        let _ = self.reader.read_line(&mut read_line);
        while read_line.len() > 0 {
            let game_match = regex("game").find(&read_line).unwrap().unwrap();
            let game_number = game_match.as_str().parse::<i32>().unwrap();
            let color_matches = regex("digit_color").captures_iter(&read_line);
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
            read_line.clear();
            let _ = self.reader.read_line(&mut read_line);
        }
        Ok(aggreg.to_string())
    }

    fn part_two(&mut self) -> Result<String, Box<dyn Error>> {
        let mut aggreg = 0;
        let mut read_line = String::new();
        let _ = self.reader.read_line(&mut read_line);
        while read_line.len() > 0 {
            let color_matches = regex("digit_color").captures_iter(&read_line);
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
            read_line.clear();
            let _ = self.reader.read_line(&mut read_line);
            aggreg += red_max * green_max * blue_max;
        }
        Ok(aggreg.to_string())
    }
}
