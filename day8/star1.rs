use std::fs::File;
use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};



const BASE: u64 = 13;


fn main() -> std::io::Result<()> {
    let mut lefts: HashMap<String, String> = HashMap::new();
    let mut rights: HashMap<String, String> = HashMap::new();

    let file = File::open("day8/input.txt")?;
    let reader = io::BufReader::new(file);

    // Collect lines into a Vec<String>
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    let instructions = &lines[0];
    let map = &lines[2..];

    for line in map {
        let words: Vec<&str> = line.split_whitespace().collect();
        let base = words[0].to_string();
        let left = words[2][1..4].to_string();
        let right = words[3][0..3].to_string();
        lefts.insert(base.clone(), left);
        rights.insert(base, right);
    }
    

    let mut steps = 0;
    let mut curr: String = "AAA".to_string();
    while true == true {

        for dir in instructions.chars() {
            if dir == 'R' {curr = rights[&curr].clone();}
            if dir == 'L' {curr = lefts[&curr].clone();}
            steps += 1;
            if curr == "ZZZ" {
                println!("done in {} steps", steps);
                return Ok(());
            }

        }
    }

    return Ok(());
}
