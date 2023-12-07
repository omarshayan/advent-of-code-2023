use std::fs;

fn roots(a: i64, b: i64, c: i64) -> (f64, f64) {
    let sqrt_discriminant = ((b * b - 4 * a * c) as f64).sqrt();
    let root1 = (-1 * b) as f64 - sqrt_discriminant;
    let root2 = (-1 * b) as f64 + sqrt_discriminant;

    return (root1/(-2.0), root2/(-2.0));
     
}

fn main() -> std::io::Result<()> {
    // Specify the file path
    let file_path = "day6/input.txt";
    let content = fs::read_to_string(file_path)?;

    let lines: Vec<String> = content.split("\n").map(String::from).collect();

    println!("{:?}", lines);

    let races: Vec<Vec<i64>> = lines.iter().map(|s| s.split_whitespace().skip(1).map(|el| el.parse::<i64>().unwrap()).collect()).collect();

    println!("{:?}", races);
    //let mut sum: u64 = 1;

    for race in 0..(races[0].len()) {
        let race_length = races[0][race];
        let race_record = races[1][race];
        //println!("length: {} \t, record: {}", race_length, race_record);
        let charge_times = roots(-1, race_length, -1 * race_record);
        //println!("root1: {} \t, root2: {}", charge_times.0, charge_times.1);
        let good_charge_times = ((charge_times.0 - 1.0).ceil() as u64, (charge_times.1 + 1.0).floor() as u64);
        //println!("chargetime1: {} \t, chargtime2: {}", good_charge_times.0, good_charge_times.1);
    
        let ways2win: u64 = good_charge_times.0 - good_charge_times.1 + 1;
        println!("ways2win: {}", ways2win);
        //sum *= ways2win;

    }

    //println!("answer: {}", sum);
    
    

    Ok(())
}
