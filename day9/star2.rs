use std::fs::File;
use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};


fn main() -> std::io::Result<()> {

    let file = File::open("day9/input.txt")?;
    let reader = io::BufReader::new(file);

    // Collect lines into a Vec<String>
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    let mut next_vals_sum: i32 = 0;

    for line in lines.iter() {

        let starter_sequence: Vec<i32> = line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
        

        let mut curr: Vec<i32> = starter_sequence;
        let mut next: Vec<i32> = Vec::new();

        let mut history: Vec<Vec<i32>> = Vec::new();

        history.push(curr.clone());

        while !curr.iter().all(|&x| x == 0) {
            for i in 0..curr.len()-1 {
                next.push(curr[i+1] - curr[i]);
            }
            history.push(next.clone());
            curr = next.clone();
            next.clear();

        }

        for seq in &history {
            println!("{:?}", seq);
        }
        println!();
        for i in (0..history.len()-1).step_by(2) {
            next_vals_sum += history[i][0];
            next_vals_sum -= history[i+1][0];
        }
    }
    println!("answer: {}", next_vals_sum);

    return Ok(());
}
