//! YEAR:   2023
//! DAY:    13

use crate::prelude::*;

impl Answers for Day {
    fn part_one(&mut self) -> Result<String, Box<dyn Error>> {
        let valley = Valley::new(&self.input);
        Ok(valley.count_mirrors(false).to_string())
    }

    fn part_two(&mut self) -> Result<String, Box<dyn Error>> {
        let valley = Valley::new(&self.input);
        Ok(valley.count_mirrors(true).to_string())
    }
}

struct Valley {
    sections: Vec<Vec<Vec<char>>>,
}

impl Valley {
    fn new(input: &str) -> Self {
        let mut section = vec![];
        let mut sections = vec![];
        for line in input.lines() {
            if line.len() == 0 {
                sections.push(section.clone());
                section.clear();
            } else {
                section.push(line.chars().collect::<Vec<char>>());
            }
        }
        sections.push(section.clone());
        Valley { sections: sections }
    }

    fn count_mirrors(&self, smudge: bool) -> usize {
        let mut x = 0;
        let mut y = 0;
        for section in &self.sections {
            let mut mid = find_palindromes(section);
            if smudge {
                mid = find_smudge(section, mid);
            }
            if mid > 0 {
                x += mid;
            } else {
                let transpose = transpose(section);
                mid = find_palindromes(&transpose);
                if smudge {
                    mid = find_smudge(&transpose, mid)
                }
                y += mid
            }
        }
        return x + 100 * y;
    }
}

fn find_smudge(section: &Vec<Vec<char>>, original: usize) -> usize {
    let width = section[0].len();
    let length = section.len();
    let mut smugde = 0;
    '_mid: for mid in 1..width {
        for j in 0..length {
            for i in 0..(width - mid).min(mid) {
                if section[j][mid - i - 1] != section[j][mid + i] {
                    smugde += 1;
                }
            }
        }
        if smugde > 1 {
            smugde = 0;
            continue '_mid;
        }
        if mid != original {
            return mid;
        }
    }
    return 0;
}

fn find_palindromes(section: &Vec<Vec<char>>) -> usize {
    let width = section[0].len();
    let length = section.len();
    '_mid: for mid in 1..width {
        for j in 0..length {
            for i in 0..(width - mid).min(mid) {
                if section[j][mid - i - 1] != section[j][mid + i] {
                    continue '_mid;
                }
            }
        }
        return mid;
    }
    return 0;
}

fn transpose(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut transpose = vec![];
    for c in 0..matrix[0].len() {
        let mut new_vec = vec![];
        for t in 0..matrix.len() {
            new_vec.push(matrix[t][c])
        }
        transpose.push(new_vec);
    }
    return transpose;
}
