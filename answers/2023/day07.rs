//! YEAR:   2023
//! DAY:    07

use crate::prelude::*;
use std::{cmp::Ordering, collections::HashMap};

lazy_static! {
    static ref ORDER: [char; 13] =
        ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];
    static ref DISORDER: [char; 13] =
        ['J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'];
}

impl Answers for Day {
    fn part_one(&mut self) -> Result<String, Box<dyn Error>> {
        Ok(self.deal_cards(false))
    }

    fn part_two(&mut self) -> Result<String, Box<dyn Error>> {
        Ok(self.deal_cards(true))
    }
}

trait Dealer {
    fn deal_cards(&mut self, intersting: bool) -> String;
}

impl Dealer for Day {
    fn deal_cards(&mut self, interesting: bool) -> String {
        let mut hand_string = String::new();
        let _ = self.reader.read_line(&mut hand_string);
        let mut hands = Hands::new();
        while hand_string.len() > 0 {
            let hand = hand_string
                .split(" ")
                .map(|s| s.trim())
                .collect::<Vec<&str>>();
            hands.push(&hand, interesting);
            hand_string.clear();
            _ = self.reader.read_line(&mut hand_string);
        }
        hands.tally_hands().to_string()
    }
}

struct Hands {
    data: Vec<ScoredHand>,
}
struct ScoredHand {
    bid: i32,
    hand_score: i32,
    card_scores: Vec<i32>,
}

trait Rank {
    fn new() -> Self;
    fn push(&mut self, hand: &Vec<&str>, omoshiroi: bool);
    fn score_hand(&self, hand: &str, omoshiroi: bool) -> i32;
    fn score_cards(&self, hand: &str, omoshiroi: bool) -> Vec<i32>;
    fn comp_scores(&self, hand1: &ScoredHand, hand2: &ScoredHand) -> Ordering;
    fn tally_hands(&self) -> usize;
}

impl Rank for Hands {
    fn new() -> Self {
        Hands { data: vec![] }
    }

    fn push(&mut self, hand: &Vec<&str>, omoshiroi: bool) {
        let hand_score = self.score_hand(&hand[0], omoshiroi);
        let card_scores = self.score_cards(&hand[0], omoshiroi);
        let scored_hand = ScoredHand {
            bid: hand[1].parse::<i32>().unwrap(),
            hand_score,
            card_scores,
        };
        let idx = self
            .data
            .binary_search_by(|item| self.comp_scores(item, &scored_hand));
        let position;
        match idx {
            Ok(idx) => {
                position = idx + 1;
            }
            Err(idx) => {
                position = idx;
            }
        }
        self.data.insert(position, scored_hand);
    }

    fn score_hand(&self, hand: &str, omoshiroi: bool) -> i32 {
        let mut card_map = HashMap::new();
        for card in hand.chars() {
            let count = card_map.entry(card).or_insert(0);
            *count += 1;
        }
        if omoshiroi {
            if let Some(joker) = card_map.remove(&'J') {
                let (card, _max);
                let card_max = card_map.iter().max_by_key(|&(_, value)| value);
                if card_max.is_some() {
                    (card, _max) = card_max.unwrap()
                } else {
                    card = &'J';
                }
                let count = card_map.entry(*card).or_insert(0);
                *count += joker;
            }
        }
        let mut score = 0;
        for val in card_map.values() {
            score += i32::pow(*val, 2);
        }
        score
    }

    fn score_cards(&self, hand: &str, omoshiroi: bool) -> Vec<i32> {
        let scores;
        if omoshiroi {
            scores = hand
                .chars()
                .map(|c| DISORDER.iter().position(|co| co == &c).unwrap() as i32)
                .collect::<Vec<i32>>();
        } else {
            scores = hand
                .chars()
                .map(|c| ORDER.iter().position(|co| co == &c).unwrap() as i32)
                .collect::<Vec<i32>>();
        }
        scores
    }

    fn comp_scores(&self, hand1: &ScoredHand, hand2: &ScoredHand) -> Ordering {
        if hand1.hand_score != hand2.hand_score {
            return hand1.hand_score.cmp(&hand2.hand_score);
        } else {
            for (card_score1, card_score2) in hand1.card_scores.iter().zip(hand2.card_scores.iter())
            {
                if card_score1 != card_score2 {
                    return card_score1.cmp(card_score2);
                }
            }
            hand1.hand_score.cmp(&hand2.hand_score)
        }
    }

    fn tally_hands(&self) -> usize {
        return self.data
            .iter()
            .enumerate()
            .fold(0, |acc, hand| acc + (hand.0 + 1) * hand.1.bid as usize);
    }
}
