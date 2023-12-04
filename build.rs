use std::{env, fs};

fn main() {
    let current_day_answser_path = env::var("CURRENT_DAY_ANS_PATH").unwrap();
    fs::remove_file(&current_day_answser_path).unwrap();
    fs::copy("src/answer.rs", &current_day_answser_path).expect("Failed to copy answers.");
}