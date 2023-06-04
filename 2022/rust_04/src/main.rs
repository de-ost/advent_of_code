// https://adventofcode.com/2022/day/4

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("resources/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut result = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        let elfs = line.split(",");

        let elf_group: Vec<&str> = elfs.collect();

        if one_contains_other(elf_group).unwrap() {
            result += 1;
        }
    }

    println!("Result: {}", result);
}

fn one_contains_other(elf_group: Vec<&str>) -> Result<bool, &'static str> {
    if elf_group.len() != 2 {
        return Err("Too many Elfs");
    }

    let mut cleaning_range: Vec<(i32, i32)> = Vec::new();

    for elf in elf_group {
        let elf: (i32, i32) = {
            let mut range = elf.split("-");
            (
                range.next().unwrap().parse().unwrap(),
                range.next().unwrap().parse().unwrap(),
            )
        };

        cleaning_range.push(elf);
    }

    cleaning_range.sort_by(|a, b| {
        if a.0 == b.0 {
            b.1.cmp(&a.1)
        } else {
            a.0.cmp(&b.0)
        }
    });

    if cleaning_range[0].1 >= cleaning_range[1].1 {
        Ok(true)
    } else {
        Ok(false)
    }
}
