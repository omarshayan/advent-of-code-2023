use std::fs;
use std::collections::HashSet;

fn main() -> std::io::Result<()> {
    // Specify the file path
    let file_path = "day4/input.txt";

/*
    // Read the contents of the file into a string
    let input: Vec<&str> = fs::read_to_string(file_path).unwrap().lines().collect();

    for card in input.iter() {
    // Read the file into a String
*/
    let file_content = fs::read_to_string(file_path).unwrap();

    // Create a Vec<&str> by splitting the file content into lines
    let input: Vec<&str> = file_content.lines().collect();
    let mut num_cards: Vec<u32> = vec![1; input.len()];
    let mut idx = 1;


    let mut tot = 0;

    for &card in &input {
        let mut sum = 0;
            
        let mut num_winning: u32 = 0;

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
                num_winning += 1;
            }
        }
        for draw in draws.iter() {
            if winning.contains(draw) {
                if sum == 0 {sum += 1;}
                else {sum *=2};
            }
        }

        for win in 1..=num_winning {
            if idx + (win as usize) > input.len() {continue;};
            num_cards[(idx + (win as usize)) - 1] += num_cards[idx-1];
        }

        // Display the contents of the file
        println!("card: {} , \tnum winning: {}", idx, num_winning);
        println!("res: {:?}", num_cards);
        idx += 1;
        tot += sum;
    }

    let res: u32 = num_cards.iter().sum();

    println!("res: {:?}", num_cards);
    println!("res: {}", res);
    println!("total: {}", tot); 


    Ok(())
}
