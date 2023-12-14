use std::fs::File;
use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

mod grid_utils;

fn main() -> std::io::Result<()> {

    let file = File::open("day10/input.txt")?;
    let reader = io::BufReader::new(file);

    // Collect lines into a Vec<String>
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    let grid: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
    let mut gridcp = grid.clone();

    let mut start_pos: (i32, i32) = (0, 0);
    let mut num_dot_total = 0;

    for row in 0..grid.len() {
        for ch in 0..grid[row].len() {
            match grid[row][ch] {
                'S' => { start_pos = (row as i32, ch as i32); }
                '.' => { num_dot_total += 1; }
                _ => {}
            }
//            if row == 0 || row == grid.len()-1 || ch == 0 || ch == grid[row].len() {edges.push((row as i32, ch as i32));}
        }
    }



    // DEFINE PIPE
    let mut prev_pos: (i32, i32) = start_pos;
    //let mut next_to_start = (49, 95);
    let mut next_to_start = (50, 96);
    //let mut next_to_start = (5, 12);
    //let mut next_to_start = (5, 12);
    let mut curr_pos: (i32, i32) = next_to_start;
    let mut next_pos1: (i32, i32) = next_to_start;
    let mut next_pos2: (i32, i32) = next_to_start;
    let mut next_pos: (i32, i32) = next_to_start;

    let mut pipe: HashSet<(i32, i32)> = HashSet::new();
    let mut inside: HashSet<(i32, i32)> = HashSet::new();
    let mut outside: HashSet<(i32, i32)> = HashSet::new();
    let mut pipe_connects: HashMap<(i32, i32), HashSet<(i32, i32)>> = HashMap::new();
    let mut inner_seeds: Vec<(i32, i32)> = Vec::new();
    let mut outer_seeds: Vec<(i32, i32)> = Vec::new();
    let mut pipe_len = 1;
    let mut num_inside = 0;
    let mut num_outside = 0;
    pipe.insert(next_to_start);

    let mut loop_end = false;
    while !loop_end {
        match grid[curr_pos.0 as usize][curr_pos.1 as usize] {
            '7' => {next_pos1 = (curr_pos.0 + 1, curr_pos.1 ); next_pos2 = (curr_pos.0, curr_pos.1 - 1);}
            'J' => {next_pos1 = (curr_pos.0 - 1, curr_pos.1 ); next_pos2 = (curr_pos.0, curr_pos.1 - 1);}
            'L' => {next_pos1 = (curr_pos.0 - 1, curr_pos.1 ); next_pos2 = (curr_pos.0, curr_pos.1 + 1 );}
            'F' => {next_pos1 = (curr_pos.0 + 1 , curr_pos.1 ); next_pos2 = (curr_pos.0, curr_pos.1 + 1);}
            '|' => {next_pos1 = (curr_pos.0 + 1, curr_pos.1 ); next_pos2 = (curr_pos.0 - 1, curr_pos.1 );}
            '-' => {next_pos1 = (curr_pos.0, curr_pos.1 + 1); next_pos2 = (curr_pos.0, curr_pos.1 - 1);}
            'S' => {
                    }
            _   => {}
            }

        if next_pos1 == prev_pos {next_pos = next_pos2} else {next_pos = next_pos1;}
        
        pipe_connects.entry(curr_pos.clone()).or_insert_with(HashSet::new).insert(next_pos.clone());
        pipe_connects.entry(next_pos.clone()).or_insert_with(HashSet::new).insert(curr_pos.clone());

        // check for inner seeds
        let mut neighbs = grid_utils::get_neighbs(curr_pos.0, curr_pos.1, &grid);
        println!("neighbs");
        for el in &neighbs {
            println!("{}, {}", el.0, el.1);
        }
        println!("curr : {}, {}", curr_pos.0, curr_pos.1);
        println!("next : {}, {}", next_pos.0, next_pos.1);
        let next_dir_idx = neighbs.iter().position(|&pos| pos == next_pos).unwrap();
        let mut swapped_side = false;
        for i in 1..neighbs.len() {
            //let idx = (next_dir_idx + neighbs.len() - i) % neighbs.len();
            let idx = (next_dir_idx + i) % neighbs.len();
            println!("checkin idx {} in {:?}", idx, neighbs);
            if neighbs[idx] == prev_pos {
                swapped_side = true;
                continue;
            } else if !swapped_side && !inner_seeds.contains(&neighbs[idx]){
                inner_seeds.push(neighbs[idx]);
                num_inside += 1;
            } else if swapped_side && !outer_seeds.contains(&neighbs[idx]){
                outer_seeds.push(neighbs[idx]);
                num_outside += 1;
            }

        }

        prev_pos = curr_pos.clone();
        curr_pos = next_pos.clone();
        pipe_len +=1;
        pipe.insert(curr_pos);
        if grid[curr_pos.0 as usize][curr_pos.1 as usize] == 'S' {
            loop_end = true;
        }
    }
    inner_seeds.retain(|&pos| !(pipe.contains(&pos)));
    outer_seeds.retain(|&pos| !(pipe.contains(&pos)));

    num_inside = inner_seeds.len() as i32;
    num_outside = outer_seeds.len() as i32;
    // FLOOD FILL
    let mut q: Vec<(i32, i32)> = Vec::new();
    let mut seen: HashSet<(i32, i32)> = HashSet::new();
    
 
    println!("start idx: {}, {}", start_pos.0, start_pos.1);
    println!("pipe set: ");
    for el in &pipe {
        println!("{}, {}", el.0, el.1);
    }
    println!("nner seed set: ");
    for el in &inner_seeds {
        println!("{}, {}", el.0, el.1);
        seen.insert(*el);
        q.push(*el);
        inside.insert(*el);
    }
    println!("size: {}", inner_seeds.len());

    

    while q.len() != 0 {
        let curr = q.pop().unwrap();
        let neighbs: Vec<(i32, i32)> = grid_utils::get_neighbs(curr.0, curr.1, &grid);
        for neighb in neighbs {
            if seen.contains(&neighb) {continue;}
            seen.insert(neighb);
            if !pipe.contains(&neighb) { 
                q.push(neighb);
                num_inside += 1;
                inside.insert(neighb);
            }
        }
    }
    println!("num inside:{}" , num_inside as i32);

    let mut q: Vec<(i32, i32)> = Vec::new();
    let mut seen: HashSet<(i32, i32)> = HashSet::new();

    println!("start idx: {}, {}", start_pos.0, start_pos.1);
    println!("pipe set: ");
    for el in &pipe {
        println!("{}, {}", el.0, el.1);
    }
    println!("outer seed set: ");
    for el in &outer_seeds {
        println!("{}, {}", el.0, el.1);
        seen.insert(*el);
        q.push(*el);
        outside.insert(*el);

    }
    println!("size: {}", inner_seeds.len());

    while q.len() != 0 {
        let curr = q.pop().unwrap();
        let neighbs: Vec<(i32, i32)> = grid_utils::get_neighbs(curr.0, curr.1, &grid);
        for neighb in neighbs {
            if seen.contains(&neighb) {continue;}
            seen.insert(neighb);
            if !pipe.contains(&neighb) { 
                q.push(neighb);
                outside.insert(neighb);
                num_outside += 1;
            }
        }
    }


//    edges.retain(|&pos| !(pipe.contains(&pos)));


    for row in 0..grid.len() {
        for ch in 0..grid[row].len() {
            if (pipe.contains(&(row as i32, ch as i32))) {
                gridcp[row][ch] = 'X';
                print!("{}", grid[row][ch]);
            } else if outside.contains(&(row as i32, ch as i32)){
                gridcp[row][ch] = ' ';
                print!("O");
            } else if inside.contains(&(row as i32, ch as i32)){
                gridcp[row][ch] = ' ';
                print!("I");
            } else {print!(" ");}
        }
        println!();
    }


    println!("num in pipe:{}", pipe.len() as i32);
    println!("num inside:{}", inside.len() as i32);
    println!("num outside:{}", outside.len() as i32);
    println!("total: {}", grid.len() * grid[0].len());
    println!("should be 0: {}", grid.len()  as i32* grid[0].len() as i32 - pipe.len() as i32 - inside.len() as i32 - outside.len() as i32);

    return Ok(());
}
