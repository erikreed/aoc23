use std::collections::BTreeMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

const PART1: bool = false;

fn to_rank(c: char) -> u8 {
    // A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2.
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => if PART1 { 11 } else { 1 },
        'T' => 10,
        _ => c.to_digit(10).unwrap() as u8
    }
}

fn to_score(hand: &str) -> (i32, i32) {
    let mut counts = BTreeMap::new();
    for c in hand.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    if !PART1 {
        let j_count = counts.remove(&'J').unwrap_or(0);
        let max_char = counts.iter()
            .max_by_key(|(&k, &v)| v)
            .map(|(k, v)| k)
            .unwrap_or(&'J');
        counts.entry(*max_char).and_modify(|c| {
            *c += j_count;
        }).or_insert(j_count);
    }

    let counts: Vec<i32> = counts.into_values().collect();
    let score;
    ;
    if counts.contains(&5) {
        score = 6;
    } else if counts.contains(&4) {
        score = 5;
    } else if counts.contains(&3) {
        score = if counts.contains(&2) { 4 } else { 3 };
    } else {
        score = match counts.iter().filter(|&&c| c == 2).count() {
            2 => 2,
            1 => 1,
            _ => 0
        };
    }
    let ranks: Vec<u8> = hand.chars().map(to_rank).collect();
    // if ranking by highest card
    // if score == 0 {
    //     score -= *ranks.iter().max().unwrap() as i32 - 14;
    // }
    let rank_score = ranks.iter()
        .enumerate()
        .map(|(i, c)| 15_i32.pow(4 - i as u32) * *c as i32)
        .sum();
    return (score, rank_score);
}

fn main() -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(File::open("d7-p1.txt")?);

    let mut hands: Vec<(String, u32)> = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        let (hand, bid) = line.split_once(' ').unwrap();
        let bid: u32 = bid.parse().unwrap();
        hands.push((hand.to_string(), bid));
    }
    hands.sort_by_key(|(hand, bid)| to_score(hand));

    let mut part1_sum = 0;
    for (i, (_, bid)) in dbg!(&hands).iter().enumerate() {
        part1_sum += bid * (i as u32 + 1);
    }

    println!("part1-{PART1}: {part1_sum}");
    return Ok(());
}