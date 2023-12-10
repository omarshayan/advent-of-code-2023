use std::fs::File;
use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};



const BASE: u64 = 13;

fn card2num(cards: &str) -> u64 {
    let card_values: &HashMap<char, u64> = &{
        [
            ('J', 0),
            ('2', 1),
            ('3', 2),
            ('4', 3),
            ('5', 4),
            ('6', 5),
            ('7', 6),
            ('8', 7),
            ('9', 8),
            ('T', 9),
            ('Q', 10),
            ('K', 11),
            ('A', 12),
        ]
        .iter()
        .cloned()
        .collect()
    };
    let mut sum: u64 = 0;
    let mut card_counts: HashMap<char, u32> = HashMap::new();
    let mut idx: u32 = 0;

    let mut seen_sets = HashSet::new();
    println!();
    card_counts.insert('J', 0);

    for card in cards.chars() {
        println!("evaluating {}", card);
        let val = (*card_values.get(&card).unwrap() as u64) * BASE.pow(cards.len() as u32 - 1 - idx);
        sum += val;
        println!("power: {}\t adding {}", cards.len() as u32 - 1 - idx, val);
        *card_counts.entry(card).or_insert(0) += 1;
        idx += 1;
    }

    let mut jokers_used = false;

    for i in (2..=5).rev() {
        println!("power: {}", idx + i - 2);
        for (card, count) in card_counts.iter() {
            if *count == i && !seen_sets.contains(card) && card != &'J'{
                let mut groupsize: u32 = i;
                if !jokers_used {
                    groupsize += card_counts[&'J'];
                    jokers_used = true;
                }
                println!("\t group of: {}, worth {}", groupsize, BASE.pow(idx+groupsize-2 as u32));
                seen_sets.insert(*card);
                sum += BASE.pow(idx+groupsize-2 as u32);
            }
        }
    }
    if !jokers_used && card_counts[&'J'] != 0 {
        sum += BASE.pow(idx-2 + (card_counts[&'J']+1).min(5));
        println!("jokers unused, adding group of {} worth {}", (card_counts[&'J']+1).min(5), BASE.pow(idx-2 + (card_counts[&'J']+1).min(5)) );
    }

    println!("sum: {}", sum);
    sum
}

fn main() -> std::io::Result<()> {
    let file = File::open("day7/input.txt")?;
    let reader = io::BufReader::new(file);

    // Collect lines into a Vec<String>
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    println!("lines: {:?}", lines);

    let cards: Vec<&str> = lines.iter().map(|s| s.split_whitespace().next().unwrap()).collect();
    let bidstr: Vec<&str> = lines.iter().map(|s| s.split_whitespace().nth(1).unwrap()).collect();
    let bids: Vec<u64> = bidstr.iter().map(|s| s.parse::<u64>().unwrap()).collect();

    let card_vals: Vec<u64> = cards.iter().map(|card| card2num(card)).collect();

    println!("card_vals: {:?}", card_vals);

    let mut indexed_card_vals: Vec<(usize, u64)> = card_vals.iter().cloned().enumerate().collect();

    println!("card idxs: {:?}", indexed_card_vals.iter().map(|&(idx, _)| idx).collect::<Vec<usize>>());


    indexed_card_vals.sort_by_key(|&(_, value)| value);

    //indexed_card_vals = indexed_card_vals.into_iter().rev().collect();

    println!("sorted: {:?}", indexed_card_vals.iter().map(|&(_, value)| value).collect::<Vec<u64>>());
    println!("sorted idxs: {:?}", indexed_card_vals.iter().map(|&(idx, _)| idx).collect::<Vec<usize>>());

    let sorted_idx = indexed_card_vals.iter().map(|&(idx, _)| idx).collect::<Vec<usize>>();


    let mut sum = 0;
    for i in 0..cards.len() {
        println!("card {} has rank {}", cards[sorted_idx[i]], i+1);
        sum += (i+1) as u64 * bids[sorted_idx[i]];
    }

    println!("winnings : {}", sum);



    return Ok(());
}
