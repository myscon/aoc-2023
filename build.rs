use std::{env, fs};

fn main() {
    let answser_path = env::var("CURRENT_DAY_ANS_PATH").unwrap();
    fs::remove_file(&answser_path).unwrap();
    fs::copy("src/answer.rs", &answser_path).expect("Failed to copy answers.");
}