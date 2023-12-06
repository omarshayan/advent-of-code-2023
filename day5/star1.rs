use std::fs;

fn main() -> std::io::Result<()> {
    // Specify the file path
    let file_path = "day5/input.txt";
    let content =fs::read_to_string(file_path)?;

    let mut maps: Vec<String> = content.split("\n\n").map(String::from).collect();

    let seedstr: String = maps.remove(0).split(":").nth(1).unwrap().to_string();
    let mut seeds: Vec<i64> = seedstr.split_whitespace().map( |s| s.parse::<i64>().unwrap()).collect();

    println!("{:?}", seeds);
    for map in maps {
        println!("{}", map);
        let mut next: Vec<i64> = vec![-1; seeds.len()];

        for row in map.split("\n").skip(1) {
            let nums: Vec<i64> = row
                .split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect();
            if let [dest_start, source_start, range_len] = nums.as_slice() {
                
                for i in 0..seeds.len() {
                    if (seeds[i] >= *source_start) && seeds[i] < (*source_start + *range_len) && next[i] == -1 {
                        next[i] = *dest_start + seeds[i] - *source_start;
                    }
                }
            }
        }

        for i in 0..seeds.len() {if next[i] == -1 {next[i] = seeds[i];}}
        seeds = next;
        println!("{:?}", seeds);
    }
    println!("solution: {}", seeds.iter().min().unwrap());

    Ok(())
}
