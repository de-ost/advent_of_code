// https://adventofcode.com/2022/day/4

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("resources/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut result_contains = 0;
    let mut result_overlapping = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        let elfs = line.split(",");
        let elf_group: Vec<&str> = elfs.collect();
        let sorted_ranges = sorted_ranges(&elf_group);

        if one_contains_other(&sorted_ranges).unwrap() {
            result_contains += 1;
        }
        if overlapping(&sorted_ranges).unwrap() {
            result_overlapping += 1;
        }
    }

    println!("Result containment: {}", result_contains);
    println!("Result overlapping: {}", result_overlapping);
}

fn sorted_ranges(elf_group: &Vec<&str>) -> Vec<(i32, i32)> {
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
    cleaning_range
}

// Part I
fn one_contains_other(cleaning_range: &Vec<(i32, i32)>) -> Result<bool, &'static str> {
    if cleaning_range.len() != 2 {
        return Err("Too many Elfs");
    }

    if cleaning_range[0].1 >= cleaning_range[1].1 {
        Ok(true)
    } else {
        Ok(false)
    }
}

// Part II
fn overlapping(cleaning_range: &Vec<(i32, i32)>) -> Result<bool, &'static str> {
    if cleaning_range.len() != 2 {
        return Err("Too many Elfs");
    }

    if cleaning_range[0].1 >= cleaning_range[1].0 {
        Ok(true)
    } else {
        Ok(false)
    }
}
