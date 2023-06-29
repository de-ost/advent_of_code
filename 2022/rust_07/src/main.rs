// https://adventofcode.com/2022/day/7

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const AT_MOST: usize = 100000;

mod ffile;
mod folder;

use folder::CWD;

fn main() {
    let file = File::open("resources/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut cwd = CWD::new();

    // read file
    for line in reader.lines() {
        let line = line.unwrap();

        let split: Vec<&str> = line.split_ascii_whitespace().collect();
        let mut commands = split.iter();

        let first_command = *commands.next().unwrap();

        match first_command {
            "$" => process_command(commands.as_ref(), &mut cwd),
            "dir" => cwd.add_folder(commands.next().unwrap()),
            "ls" => (),
            _ => {
                let size = first_command.parse::<usize>().unwrap();
                let name = commands.next().unwrap();

                cwd.add_file(name, size);
            }
        }
    }

    // Calculate sum
    cwd.change_directory("/").unwrap();
    let sum = sum_up(&cwd, AT_MOST);

    println!("Total sum: {}", sum);
}

fn process_command(inp: &[&str], cwd: &mut CWD) {
    let mut iter = inp.into_iter();

    let first = *iter.next().unwrap();

    match first {
        "cd" => {
            let second = *iter.next().unwrap();
            cwd.change_directory(second).unwrap()
        }
        "ls" => (),
        _ => todo!(),
    }
}

fn sum_up(cwd: &CWD, max_size: usize) -> u32 {
    let mut sum = 0;

    for id in 0..cwd.folders.len() {
        let size = cwd.get_size(id);

        if size <= max_size {
            sum += size as u32;
        }
    }

    sum
}
