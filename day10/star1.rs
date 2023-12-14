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

    let mut start_pos: (i32, i32) = (0, 0);

    for row in 0..grid.len() {
        for ch in 0..grid[row].len() {
            if grid[row][ch] == 'S' { start_pos = (row as i32, ch as i32); }
        }
    }
    
    let mut curr_pos: (i32, i32) = start_pos;
    let mut prev_pos: (i32, i32) = curr_pos;
    let mut neighb_found = false;

    let mut next_pos1: (i32, i32) = start_pos;
    let mut next_pos2: (i32, i32) = start_pos;
    let mut next_pos: (i32, i32) = start_pos;

    println!("start idx: {}, {}", start_pos.0, start_pos.1);

    let neighbs: Vec<((i32, i32), char)> = grid_utils::get_neighbs(curr_pos.0, curr_pos.1, &grid);
    for neighb in neighbs {
        println!("\tneighb {} pos {}, {}", neighb.1, curr_pos.0 + neighb.0.0, curr_pos.1 + neighb.0.1);
        neighb_found = false;
        if (curr_pos.0 + neighb.0.0, curr_pos.1 + neighb.0.1) == prev_pos {continue;}
        match neighb.1 {
            '7' => if neighb.0 == (0, 1) || neighb.0 == (-1, 0) {neighb_found = true;}
            'J' => if neighb.0 == (0, 1) || neighb.0 == (1, 0) {neighb_found = true;}
            'L' => if neighb.0 == (0, -1) || neighb.0 == (1, 0) {neighb_found = true;}
            'F' => if neighb.0 == (0, -1) || neighb.0 == (-1, 0) {neighb_found = true;}
            '|' => if neighb.0 == (-1, 0) || neighb.0 == (1, 0) {neighb_found = true;}
            '-' => if neighb.0 == (0, -1) || neighb.0 == (0, 1) {neighb_found = true;}
            _   => {}
        }
        if neighb_found { 
            prev_pos = curr_pos.clone(); 
            curr_pos = (curr_pos.0 + neighb.0.0, curr_pos.1 + neighb.0.1);
            break;
        }
    }



    let mut pipe_len = 1;
    while true {
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
        prev_pos = curr_pos.clone();
        curr_pos = next_pos.clone();
        pipe_len +=1;
        if grid[curr_pos.0 as usize][curr_pos.1 as usize] == 'S' {
            println!("pipe_len {}", pipe_len);
            return Ok(());
        }
        /*
        println!("curr pos {}, {}", curr_pos.0, curr_pos.1);
        let neighbs: Vec<((i32, i32), char)> = grid_utils::get_neighbs(curr_pos.0, curr_pos.1, &grid);
        for neighb in neighbs {
            println!("\tneighb {} pos {}, {}", neighb.1, curr_pos.0 + neighb.0.0, curr_pos.1 + neighb.0.1);
            neighb_found = false;
            if (curr_pos.0 + neighb.0.0, curr_pos.1 + neighb.0.1) == prev_pos {continue;}
            match neighb.1 {
                '7' => if neighb.0 == (0, 1) || neighb.0 == (-1, 0) {neighb_found = true;}
                'J' => if neighb.0 == (0, 1) || neighb.0 == (1, 0) {neighb_found = true;}
                'L' => if neighb.0 == (0, -1) || neighb.0 == (1, 0) {neighb_found = true;}
                'F' => if neighb.0 == (0, -1) || neighb.0 == (-1, 0) {neighb_found = true;}
                '|' => if neighb.0 == (-1, 0) || neighb.0 == (1, 0) {neighb_found = true;}
                '-' => if neighb.0 == (0, -1) || neighb.0 == (0, 1) {neighb_found = true;}
                'S' => {
                        pipe_len +=1;
                        println!("pipe_len {}", pipe_len);
                        return Ok(());
                        }
                _   => {}
            }
            if neighb_found { 
                prev_pos = curr_pos.clone(); 
                curr_pos = (curr_pos.0 + neighb.0.0, curr_pos.1 + neighb.0.1);
                pipe_len += 1;
                break;
            }

        }
        if curr_pos == start_pos {break;}
        */

    }


    println!("pipe_len {}", pipe_len);

    return Ok(());
}
