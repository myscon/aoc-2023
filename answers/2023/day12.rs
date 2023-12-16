//! YEAR:   2023
//! DAY:    12

use std::collections::HashMap;
use std::usize;

use crate::prelude::*;
use crate::regex::regex;
use fancy_regex::Regex;
use itertools::Itertools;
use rayon::iter::ParallelBridge;
use rayon::prelude::ParallelIterator;

impl Answers for Day {
    fn part_one(&mut self) -> Result<String, Box<dyn Error>> {
        let records = Springs::new(&self.input, false);
        Ok(records.records_count.to_string())
    }

    fn part_two(&mut self) -> Result<String, Box<dyn Error>> {
        let records = Springs::new(&self.input, true);
        Ok(records.records_count.to_string())
    }
}

struct Springs {
    records_count: usize,
}

trait Records {
    fn new(input: &str, fold: bool) -> Self;
}

impl Records for Springs {
    fn new(input: &str, fold: bool) -> Self {
        let records_count = input
            .lines()
            .par_bridge()
            .map(|l| parse_line(l, fold))
            .sum();
        Springs { records_count }
    }
}

fn parse_line(line: &str, fold: bool) -> usize {
    let mut input_split = line.split(" ");
    let records = input_split.next().unwrap().to_string();
    let damaged = input_split.next().unwrap().to_string();
    if fold {
        let delim_r = "?".to_string();
        let delim_n = ",".to_string();
        let og = (0..4).fold(records.clone(), |acc, _| acc + &delim_r + &records);
        let br = (0..4).fold(damaged.clone(), |acc, _| acc + &delim_n + &damaged);
        return parse_record_but_faster(&og, &br);
        // This worked for the given example answers at least...too many edge cases
        //
        // if (og_string.chars().next().unwrap() != '#') && (og_string.chars().rev().next().unwrap() != '#') {
        //     let delim_r = "?".to_string();
        //     let og_record_f = og_string.clone() + &delim_r;
        //     let og_record_l = delim_r + &og_string;
        //     let f_sum = parse_record(&og_record_f, &broken_string);
        //     let l_sum = parse_record(&og_record_l, &broken_string);
        //     return f_sum.max(l_sum).pow(4)*og_sum;
        // } else{
        //     return og_sum.pow(5);
        // }
    } else {
        return parse_record_but_faster(&records, &damaged);
    }
}

fn parse_record_but_faster(records: &str, damaged: &str) -> usize {
    let og_record = records.chars().collect::<Vec<char>>();
    let broken_nums = damaged
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let mut memo = HashMap::new();
    return find_groups_even_faster_yet(&mut memo, &og_record, &broken_nums, 0, 0, 0);
    // return find_groups_slightly_faster(&og_record, &broken_nums);
}

#[allow(unused)]
fn find_groups_even_faster_yet(
    memo: &mut HashMap<(usize, usize, usize), usize>,
    records: &Vec<char>,
    damaged: &Vec<usize>,
    groups: usize,
    length: usize,
    index: usize,
) -> usize {
    if index == records.len() {
        if groups == damaged.len() || groups == damaged.len() - 1 && length == damaged[groups] {
            return 1;
        }
        return 0;
    } else {
        match records[index] {
            '#' => {
                if groups < damaged.len() && length + 1 <= damaged[groups] {
                    return find_groups_even_faster_yet(
                        memo,
                        records,
                        damaged,
                        groups,
                        length + 1,
                        index + 1,
                    );
                }
                return 0;
            }
            '.' => {
                if length == 0 {
                    return find_groups_even_faster_yet(
                        memo,
                        records,
                        damaged,
                        groups,
                        length,
                        index + 1,
                    );
                } else if length == damaged[groups] {
                    return find_groups_even_faster_yet(
                        memo,
                        records,
                        damaged,
                        groups + 1,
                        0,
                        index + 1,
                    );
                }
                return 0;
            }
            '?' => {
                if let Some(memo_pull) = memo.get(&(groups, length, index)){
                    return *memo_pull;
                }
                let mut arrangements = 0;
                if length == 0 {
                    arrangements += find_groups_even_faster_yet(
                        memo,
                        records,
                        damaged,
                        groups,
                        length,
                        index + 1,
                    );
                }
                if groups < damaged.len() && length < damaged[groups] {
                    arrangements += find_groups_even_faster_yet(
                        memo,
                        records,
                        damaged,
                        groups,
                        length + 1,
                        index + 1,
                    );
                }
                if groups < damaged.len() && length == damaged[groups] {
                    arrangements += find_groups_even_faster_yet(
                        memo,
                        records,
                        damaged,
                        groups + 1,
                        0,
                        index + 1,
                    );
                }
                memo.insert((groups, length, index), arrangements);
                return arrangements;
            }
            _ => unreachable!(),
        }
    }
}

// Run time is 12 hours for part two lmao.
#[allow(unused)]
fn find_groups_slightly_faster(records: &Vec<char>, damaged: &Vec<usize>) -> usize {
    let mut accum = 0;
    let mut stack = vec![];
    stack.push((0, 0, 0)); // (group count, length of current group, index)
    while !stack.is_empty() {
        let record = stack.pop().unwrap();
        if record.2 == records.len() {
            if record.0 == damaged.len() && record.1 == 0
                || record.0 == damaged.len() - 1 && record.1 == damaged[record.0]
            {
                accum += 1;
            }
        } else {
            match records[record.2] {
                '#' => {
                    if record.0 < damaged.len() && record.1 + 1 <= damaged[record.0] {
                        stack.push((record.0, record.1 + 1, record.2 + 1));
                    }
                }
                '.' => {
                    if record.1 == 0 {
                        stack.push((record.0, record.1, record.2 + 1));
                    } else if record.1 == damaged[record.0] {
                        // we've accrued enough damaged nodes before this end to match a cgroup
                        stack.push((record.0 + 1, 0, record.2 + 1));
                    }
                }
                '?' => {
                    if record.0 < damaged.len() && record.1 < damaged[record.0] {
                        // node is damaged but need more to match the next group
                        stack.push((record.0, record.1 + 1, record.2 + 1));
                    }
                    if record.0 < damaged.len() && record.1 == damaged[record.0] {
                        // node is damaged and can match the next group
                        stack.push((record.0 + 1, 0, record.2 + 1));
                    }
                    if record.1 == 0 {
                        // node is operational
                        stack.push((record.0, record.1, record.2 + 1));
                    }
                }
                _ => unreachable!(),
            }
        }
    }
    return accum;
}

#[allow(unused)]
// This never finished part two
fn find_groups_eventually(records: &str, damaged: &str) -> usize {
    let record_data = records.chars().collect::<Vec<char>>();
    let damaged_data = damaged
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let unknown_idxs = record_data
        .iter()
        .enumerate()
        .filter(|&(_, &c)| c == '?')
        .map(|(idx, _)| idx)
        .collect::<Vec<usize>>();
    let known_damaged = record_data.iter().filter(|&&c| c == '#').count();
    let rem_damaged = damaged_data.iter().sum::<usize>() - known_damaged;
    let rem_operational = unknown_idxs.len() - rem_damaged;
    let reg_win = regex("broken_window");
    // holy hell that's a lot of arrangements
    let arrangements = unknown_idxs.iter().combinations(rem_operational);
    let total = arrangements
        .par_bridge()
        .map(|pm| {
            let un = unknown_idxs
                .iter()
                .filter(|e| !pm.contains(e))
                .collect::<Vec<&usize>>();
            compare_record(&record_data, &damaged_data, pm, un, &reg_win)
        })
        .sum::<usize>();
    return total;
}

#[allow(unused)]
fn compare_record(
    records: &Vec<char>,
    damaged_data: &Vec<usize>,
    arrangement: Vec<&usize>,
    unknown_indexes: Vec<&usize>,
    reg_win: &Regex,
) -> usize {
    let mut filled_records = records.clone();
    arrangement
        .iter()
        .for_each(|&&idx| filled_records[idx] = '.');
    unknown_indexes
        .iter()
        .for_each(|&&idx| filled_records[idx] = '#');
    let damage_count = reg_win
        .find_iter(&filled_records.iter().collect::<String>())
        .map(|m| m.unwrap().as_str().len())
        .collect::<Vec<usize>>();
    if damage_count != *damaged_data {
        return 0;
    }
    return 1;
}
