use clap::Parser;
use std::{env, path::Path};

mod day;
mod answer;
mod regex;
mod prelude {
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

    let new_day = Day::new(input_path);

    if args.two {
        let part_two_ans = new_day.part_two().unwrap();
        println!("{:?}", part_two_ans);
    } else {
        let part_one_ans = new_day.part_one().unwrap();
        println!("{:?}", part_one_ans);
    }
}