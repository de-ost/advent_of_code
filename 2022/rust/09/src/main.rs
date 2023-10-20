// https://adventofcode.com/2022/day/9

use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));

    let mut head_pos = (0, 0);
    let mut tail_pos = (0, 0);

    let file = File::open("resources/input.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();

        let mut whitespaces = line.split_ascii_whitespace();

        let direction = whitespaces.next().unwrap();
        let steps: usize = whitespaces.next().unwrap().parse().unwrap();

        let mut x_direction = 0;
        let mut y_direction = 0;

        match direction {
            "L" => x_direction = -1,
            "R" => x_direction = 1,
            "U" => y_direction = 1,
            "D" => y_direction = -1,
            _ => panic!("Invalid direction input!"),
        }

        for _ in 0..steps {
            let old_head_pos = head_pos;

            // move head
            head_pos.0 += x_direction;
            head_pos.1 += y_direction;

            // move tail
            let dx: i32 = head_pos.0 - tail_pos.0;
            let dy: i32 = head_pos.1 - tail_pos.1;
            if dx.abs() > 1 || dy.abs() > 1 {
                tail_pos = old_head_pos;
                visited.insert(tail_pos);
            }
        }
    }

    println!("Number of visited positions: {}", visited.len());
}
