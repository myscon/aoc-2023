use clap::Parser;
use std::{env, path::Path, time::Instant};

mod day;
mod answer;
mod regex;
mod prelude {
    pub use std::fs;
    pub use crate::day::{Answers, Day};
    pub use lazy_static::lazy_static;
    pub use std::{error::Error, fs::File, io::BufReader, path::PathBuf};
}

use prelude::*;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    two: bool,
}

fn main() {
    let args = Args::parse();
    let current_year = env::var("CURRENT_YEAR").unwrap();
    let current_day_name = env::var("CURRENT_DAY_NAME").unwrap();
    let input_path = Path::new("inputs")
        .join(current_year)
        .join(current_day_name)
        .with_extension("txt");

    let mut new_day = Day::new(input_path);
    if args.two {
        time_method_call(&mut new_day, Day::part_two);
    } else {
        time_method_call(&mut new_day, Day::part_one);
    }
    
}

fn time_method_call<F>(day: &mut Day, method: F)
where
    F: Fn(&mut Day) -> String
{
    let start = Instant::now();
    let answer = method(day);
    let end = start.elapsed();
    println!("Answer: {{ {answer} }} took {{ {end:.2?} }} to complete.");
}