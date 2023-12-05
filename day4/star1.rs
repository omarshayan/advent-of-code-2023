use std::fs;
use std::collections::HashSet;

fn main() -> std::io::Result<()> {
    // Specify the file path
    let file_path = "day4/input.txt";

    // Read the contents of the file into a string
    let mut tot = 0;

    for card in fs::read_to_string(file_path).unwrap().lines() {
            
        let mut sum = 0;
        let numlists: Vec<&str> = card.split('|').collect();

        let winning: HashSet<u32> = numlists[0]
            .split_whitespace()
            .skip(2)
            .map(|s| s.parse().unwrap())
            .collect();

        let draws: Vec<u32> = numlists[1] 
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        for draw in draws.iter() {
            if winning.contains(draw) {
                if sum == 0 {sum += 1;}
                else {sum *=2};
            }
        }

        // Display the contents of the file
        println!("winnig: {:?}", winning);
        println!("draw: {:?}", draws);
        println!("score: {}", sum);
        tot += sum;
    }
    println!("total: {}", tot); 




    Ok(())
}
