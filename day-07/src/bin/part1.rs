use std::collections::HashMap;

use aocd::*;

#[aocd(2023, 7)]
fn main() {
    let input = input!();
    let result = part1(&input);
    dbg!(result);
}

#[derive(Debug)]
struct Hand<'a> {
    cards: &'a str,
    bid: u32,
    score: u32,
}

const SUIT: &str = "23456789TJQKA";

fn calculate_score(cards: &str) -> u32 {
    let mut card_count = HashMap::new();
    let card_score = cards.chars().enumerate().fold(0, |acc, (i, c)| {
        *card_count.entry(c).or_insert(0u32) += 1;
        acc + SUIT.find(c).unwrap() as u32 * 14u32.pow(4 - i as u32)
    });
    let mut card_count = card_count.iter().collect::<Vec<_>>();
    card_count.sort_by(|a, b| b.1.cmp(a.1));

    let hand_type = match card_count.len() {
        1 => 6, // Five of a kind
        2 => match card_count[0].1 {
            4 => 5, // Four of a kind
            3 => 4, // Full house
            _ => panic!(),
        },
        3 => match card_count[0].1 {
            3 => 3, // Three of a kind
            2 => 2, // Two pair
            _ => panic!(),
        },
        4 => 1, // One pair
        _ => 0, // High card
    };

    card_score + hand_type * 14u32.pow(5)
}

impl<'a> Hand<'a> {
    fn new(cards: &'a str, bid: u32) -> Self {
        Self {
            cards,
            bid,
            score: calculate_score(cards),
        }
    }
}

fn parse(input: &str) -> Vec<Hand> {
    input
        .lines()
        .filter_map(|line| {
            line.split_once(' ')
                .map(|(cards, bid)| Hand::new(cards, bid.parse::<u32>().unwrap()))
        })
        .collect()
}

fn part1(input: &str) -> u32 {
    let mut hands = parse(input);
    hands.sort_by(|a, b| a.score.cmp(&b.score));
    hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank as u32 + 1) * hand.bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../../example.txt");
        assert_eq!(part1(input), 6440);
    }
}
