use fancy_regex::Regex;
use lazy_static::lazy_static;
use toml::{from_str, Value, map};

lazy_static! {
    static ref REGEX_TABLE: map::Map<String, Value> = init_regex_table();
}

fn init_regex_table() -> map::Map<String, Value>{
    let patterns_toml = include_str!("regex.toml");
    let patterns_value: Value = from_str(patterns_toml).expect("Unable to parse TOML");
    let patterns_table = patterns_value
        .get("patterns")
        .and_then(Value::as_table).unwrap();
    return patterns_table.clone();
}

#[allow(unused)]
pub fn regex(pattern_key: &str) -> Regex {
    let pattern = REGEX_TABLE
        .get(pattern_key).unwrap()
        .as_str().unwrap();
    return Regex::new(pattern).expect("Invalid regex pattern key");
}
