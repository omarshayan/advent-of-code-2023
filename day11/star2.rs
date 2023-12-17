
use std::fs::File;
use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

mod grid_utils;

fn main() -> std::io::Result<()> {

    let file = File::open("day11/input.txt")?;
    let reader = io::BufReader::new(file);

    // Collect lines into a Vec<String>
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    let mut grid: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
    let mut wide_grid: Vec<Vec<char>> = Vec::new();


    let mut col_has_galaxy: HashMap<i64, bool> = HashMap::new();
    let mut row_has_galaxy: HashMap<i64, bool> = HashMap::new();

    for row in grid.clone() {for ch in row {print!("{}", ch)}; println!()}

    for row in 0..grid.len() {
            wide_grid.push(grid[row].clone()); 
        for ch in 0..grid[row].len() {
            if grid[row][ch] == '#' {
                col_has_galaxy.insert(ch as i64, true);
                row_has_galaxy.insert(row as i64, true);
            }
        }
    }




    let mut galaxy_locs: HashSet<(i64, i64)> = HashSet::new();
    let multiplier = 1000000;
    let mut row_offset = 0;
    for row in 0..grid.len() {
        let mut col_offset = 0;
        if !row_has_galaxy.contains_key(&(row as i64)) {row_offset += multiplier - 1;}
        for ch in 0..grid[row].len() {
            if !col_has_galaxy.contains_key(&(ch as i64)) {col_offset += multiplier - 1;}
            if grid[row][ch] == '#' {galaxy_locs.insert((row as i64 + row_offset, ch as i64 + col_offset));}
        }
    }

    let mut galaxy_locs_checked: HashSet<(i64, i64)> = galaxy_locs.clone();

    let mut distsum = 0;
    for galaxy in galaxy_locs {
        galaxy_locs_checked.remove(&galaxy);
            for other_galaxy in &galaxy_locs_checked {
            distsum += (galaxy.0 - other_galaxy.0).abs() + (galaxy.1 - other_galaxy.1).abs();

        }
    }
    println!("{}", distsum);
        
    return Ok(());
}
