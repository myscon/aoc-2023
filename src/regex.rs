use lazy_static::lazy_static;
use fancy_regex::Regex;

lazy_static! {
    pub static ref SYMBOL: Regex = Regex::new(r"[^\d.]").unwrap();
    pub static ref NUMBER: Regex = Regex::new(r"\d+").unwrap();
    pub static ref GEAR: Regex = Regex::new(r"\*").unwrap();
}