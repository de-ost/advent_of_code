// https://adventofcode.com/2022/day/5

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("resources/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut stacks: Vec<Vec<char>> = vec![vec![]; 9];

    for line in lines.by_ref().take(8) {
        let line = line.unwrap();

        for i in 0..9 {
            let char_idx = 4 * i + 1;
            let c = line.chars().skip(char_idx).next().unwrap();

            if c.is_ascii_uppercase() {
                stacks[i].push(c);
            }
        }
    }

    stacks.iter_mut().for_each(|stack| stack.reverse());

    for line in lines.skip(2) {
        let line = line.unwrap();
        let mut split = line.split_whitespace();
        let split_parser = split.by_ref();

        let count: usize = split_parser.skip(1).next().unwrap().parse().unwrap();
        let mut from: usize = split_parser.skip(1).next().unwrap().parse().unwrap();
        let mut to: usize = split_parser.skip(1).next().unwrap().parse().unwrap();

        from -= 1;
        to -= 1;

        for _ in 0..count {
            let value = stacks[from].pop().unwrap();
            stacks[to].push(value);
        }
    }

    for stack in stacks.iter_mut() {
        print!("{}", stack.pop().unwrap());
    }

    println!();
}
