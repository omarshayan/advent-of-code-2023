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

        let mut ranges: Vec::<(i64, i64)> = Vec::new();
        ranges.push((seeds[seedsetidx], seeds[seedsetidx] + seeds[seedsetidx+1]));
        println!("Values in  ranges:");
        for &(a, b) in &ranges {
            println!("({}, {})", a, b);
        }

        for map in all_maps.as_slice() {
            println!("map: {:?}", map);
            let mut next_map_ranges: Vec::<(i64, i64)> = Vec::new();

            for row in map {
                println!("row: {:?}", row);

                let mut next_row_ranges: Vec::<(i64, i64)> = Vec::new();
                for mut range in ranges.iter_mut() {
                    if range.0 >= row[1] && range.0 < (row[1] + row[2]) {
                        range.0 = row[0] + range.0 - row[1];   
                        if range.1 < (row[1] + row[2]) {
                           range.1 = row[0] + range.1 - row[1];   
                        } else {
                           next_row_ranges.push((row[1] + row[2], range.1));
                           range.1 = row[0] + row[2];   
                        }
                        next_map_ranges.push(range.clone());

                    } else if range.0 < row[1] && range.1 >= row[1] {
                        if range.1 < row[1] + row[2] {
                            next_map_ranges.push((row[0], row[0] + range.1 - row[1]));
                        } else {
                            next_map_ranges.push((row[0], row[0] + row[2]));
                            next_row_ranges.push((row[1] + row[2], range.1));
                        }
                        range.1 = row[1];
                        next_row_ranges.push(range.clone());
                    } else {
                        next_row_ranges.push(range.clone());
                    }
                }
                ranges = next_row_ranges.clone(); 

            }
            ranges.extend(next_map_ranges.clone());
            println!("Values in  ranges:");
            for &(a, b) in &ranges {
                println!("({}, {})", a, b);
            }
        }


        // Use iter and min functions to find the minimum value
        let min_value = ranges
            .iter()
            .flat_map(|&(a, b)| vec![a, b]) // Flatten the tuples into a single iterator
            .min();

        match min_value {
            Some(min) => {
                println!("Minimum value: {}", min);
                minloc = cmp::min(min, minloc);
            },
            None => println!("Vector is empty"),
        }

    }
    println!("FINAL value: {}", minloc);


    Ok(())
}
