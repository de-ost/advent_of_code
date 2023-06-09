// https://adventofcode.com/2022/day/6

use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() -> io::Result<()> {
    let file = File::open("resources/input.txt")?;
    let reader = BufReader::new(file);

    let mut count = 0;
    let mut buffer_count = 0;
    let mut buffer: [char; 4] = [' '; 4];

    let mut found = false;

    'search: for line in reader.lines() {
        for character in line?.chars() {
            if buffer.iter().take(buffer_count).any(|&c| c == character) {
                buffer_count = 0;
            }

            if buffer_count == 4 {
                found = true;
                break 'search;
            }

            buffer[buffer_count] = character;
            buffer_count += 1;
            count += 1;
        }
    }

    if found {
        println!("Found at index {}.", count);
    } else {
        println!("Not found.");
    }

    Ok(())
}
