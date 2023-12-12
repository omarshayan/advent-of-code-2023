use std::fs::File;
use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

fn main() -> std::io::Result<()> {
    let mut lefts: HashMap<String, String> = HashMap::new();
    let mut rights: HashMap<String, String> = HashMap::new();

    let file = File::open("day8/input.txt")?;
    let reader = io::BufReader::new(file);

    // Collect lines into a Vec<String>
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    let instructions = &lines[0];
    let map = &lines[2..];

    let mut bases: Vec<String> = Vec::new();

    for line in map {
        let words: Vec<&str> = line.split_whitespace().collect();
        let base: String = words[0].to_string();
        if base.chars().nth(2).unwrap() == 'A' {
            bases.push(base.clone());
        }
        let left = words[2][1..4].to_string();
        let right = words[3][0..3].to_string();
        lefts.insert(base.clone(), left);
        rights.insert(base, right);
    }

    let num_ghosts = bases.len();
    // let num_ghosts = 1;
    let mut currs = bases.clone();
    

    let mut steps: u32 = 0;
    let mut loop_found = false;
    let mut hash_cache: HashMap<(String, u32), u32> = HashMap::new();
    let mut dir_idx: u32 = 0;

    let mut loop_observed_at: Vec<(String, Vec<u32>)> = Vec::new();
    let mut z_observed_at: HashMap<String, Vec<u32>> = HashMap::new();

    println!("bases: {:?}", bases);

    for base in &mut bases {
        hash_cache = HashMap::new();
        z_observed_at.insert(base.to_string(), Vec::new());
        let mut curr: String = base.clone();
        let mut done = 0;
        steps = 0;
        while !loop_found {
            dir_idx = 0;
            for dir in instructions.chars() {
                if dir == 'R' {curr = rights[&curr.clone()].clone();}
                if dir == 'L' {curr = lefts[&curr.clone()].clone();}

                steps += 1;
                dir_idx += 1;
                dir_idx %= instructions.len() as u32;
                if curr.chars().nth(2).unwrap() == 'Z' {
                    done = steps;
                    if hash_cache.contains_key(&(curr.clone(), dir_idx)) {
                        loop_found = true;
                        println!("loop found, from {} steps. ", hash_cache[&(curr.clone(), dir_idx)]);
                    } else { 
                        hash_cache.insert((curr.clone(), dir_idx), steps); 
                        z_observed_at.get_mut(base).unwrap().push(steps);
                    }
                }
                    
                if loop_found {
                    //loop_observed_at.push((curr.clone(), dir_idx, steps, steps - hash_cache[&(curr.clone(), dir_idx)], done)); 
                    z_observed_at.get_mut(base).unwrap().push(steps);
                    break;
                }

                println!("curr: {} steps: {} lastdir: {} diridx: {}", curr, steps, dir, dir_idx);
                //println!("next round, {} done", done);
                //if num_ghosts == done {
                //    println!("done in {} steps", steps);
                //    return Ok(());
                //}
            }
        }
        loop_found = false;
        hash_cache.clear();
        println!("");
    }

    for (key, val) in z_observed_at {
        println!("base: {} \t z observed at : {:?}", key, val);
    }
    //for (base, loop_found) in bases.iter().zip(&loop_observed_at) {
    //    println!("base: {}\tloop start/end : {}\t dir_idx: {}\t loopsize: {}\tsteps: {}", base, loop_found.0, loop_found.1, loop_found.3, loop_found.2);
    //}

    return Ok(());
}
