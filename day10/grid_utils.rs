pub fn is_within_bounds(row: i32, col: i32, grid: &Vec<Vec<char>>) -> bool {
    let num_rows = grid.len() as i32;
    let num_cols = grid[0].len() as i32; // Assuming the grid is non-empty and rectangular

    row < num_rows && col < num_cols && row >= 0 && col >= 0
}

pub fn get_el(row: i32, col: i32, grid: &Vec<Vec<char>>) -> Option<char> {
    if is_within_bounds(row, col, grid) {
        Some(grid[row as usize][col as usize])
    } else {
        None
    }
}

pub fn get_neighbs(row: i32, col: i32, grid: &Vec<Vec<char>>) -> Vec<(i32, i32)> {
    let mut neighbs: Vec<(i32, i32)> = Vec::new();
    let dirs: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    for dir in dirs {
        if is_within_bounds(row + dir.0, col+dir.1, grid) {
            //neighbs.push(((row + dir.0, col + dir.1), grid[(row + dir.0) as usize][(col + dir.1) as usize]));
            neighbs.push((row + dir.0, col + dir.1));
        } 
    }
    neighbs
}

