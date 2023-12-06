use std::fs;
use std::cmp;

fn main() -> std::io::Result<()> {
    // Specify the file path
    let file_path = "day5/input.txt";
    let content =fs::read_to_string(file_path)?;

    let mut maps: Vec<String> = content.split("\n\n").map(String::from).collect();

    let seedstr: String = maps.remove(0).split(":").nth(1).unwrap().to_string();
    let seeds: Vec<i64> = seedstr.split_whitespace().map( |s| s.parse::<i64>().unwrap()).collect();

    println!("seeds: {:?}", seeds);

    let mut minloc = i64::MAX;

    // store the stuff
    let mut all_maps = Vec::new();
    for map in maps.as_slice() {
        let mut map_rows = Vec::new();
        for row in map.split("\n").skip(1) {
            let nums: Vec<i64> = row
                .split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect();
            map_rows.push(nums);
        }
        all_maps.push(map_rows);
    }



    for seedsetidx in (0..seeds.len()).step_by(2) {
        println!("seedset : \t{} of \t{}", seedsetidx/2, seeds.len()/2);
        for mut seed in seeds[seedsetidx]..(seeds[seedsetidx] + seeds[seedsetidx+1]) {

            for map in all_maps.as_slice() {
                //    if let [dest_start, source_start, range_len] = nums.as_slice() {
                for row in map {
                    if (seed >= row[1]) && seed < (row[1] + row[2]) {
                        seed = row[0] + seed - row[1];
                        break;
                    }
                }
            }
            minloc = cmp::min(seed, minloc);

        }
    }
    println!("solution: {}", minloc);

    Ok(())
}
