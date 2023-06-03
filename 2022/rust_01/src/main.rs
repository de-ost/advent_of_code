use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut calories_carrying: Vec<u32> = vec![0];
    
    let file = File::open("resources/input.txt").unwrap();
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();

        match line.as_str() {
            "" => calories_carrying.push(0),


            _ => {
                let calories = line.parse::<u32>().unwrap();
                if let Some(last) = calories_carrying.last_mut() {
                    *last += calories;
                }
            }
        }
    }

    calories_carrying.sort();
    calories_carrying.reverse();

    let max_val = calories_carrying.last().unwrap();
    let sum: u32 = calories_carrying
        .iter()
        .take(3)
        .sum();

    println!("MAX: {}", max_val);
    println!("SUM OF LAST 3: {}", sum);
}
