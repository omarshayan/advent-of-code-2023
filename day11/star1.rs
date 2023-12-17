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


    let mut col_has_galaxy: HashMap<i32, bool> = HashMap::new();
    let mut row_has_galaxy: HashMap<i32, bool> = HashMap::new();

    for row in grid.clone() {for ch in row {print!("{}", ch)}; println!()}

    for row in 0..grid.len() {
        let mut row_has_galaxy = false;
            wide_grid.push(grid[row].clone()); 
        for ch in 0..grid[row].len() {
            if grid[row][ch] == '#' {
                row_has_galaxy = true; 
                col_has_galaxy.insert(ch as i32, true);
                row_has_galaxy.insert(row as i32, true);
            }
        }
        if row_has_galaxy != true {
            wide_grid.push(grid[row].clone()); 
        }
    }

    grid = wide_grid;
    wide_grid = vec![vec![]; grid.len()];

    for col in 0..grid[0].len() {
        for row in 0..grid.len() {wide_grid[row].push(grid[row][col])}
        if !col_has_galaxy.contains_key(&(col as i32)) {
            for row in 0..grid.len() {wide_grid[row].push(grid[row][col])}
        }
    }
    grid = wide_grid;

    for row in &grid {for ch in row {print!("{}", ch)}; println!()}

    let mut galaxy_locs: HashSet<(i32, i32)> = HashSet::new();

    let mut row_offset = 0;
    let mut col_offset = 0;
    for row in 0..grid.len() {
        if 
        for ch in 0..grid[row].len() {
            if grid[row][ch] == '#' {galaxy_locs.insert((row as i32, ch as i32));}
        }
    }

    let mut galaxy_locs_checked: HashSet<(i32, i32)> = galaxy_locs.clone();

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
