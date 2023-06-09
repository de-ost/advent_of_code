// https://adventofcode.com/2022/day/6

use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

const START_OF_COMMUNICATION_MARKER: usize = 4;

fn main() -> io::Result<()> {
    let file = File::open("resources/input.txt")?;
    let reader = BufReader::new(file);

    let mut count = 0;
    let mut buffer_count = 0;
    let mut buffer: [char; START_OF_COMMUNICATION_MARKER] = [' '; START_OF_COMMUNICATION_MARKER];

    let mut found = false;

    'search: for line in reader.lines() {
        for character in line?.chars() {
            if buffer.iter().take(buffer_count).any(|&c| c == character) {
                buffer_count = 0;
            }

            if buffer_count == START_OF_COMMUNICATION_MARKER {
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
