// https://adventofcode.com/2022/day/3

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // Part I (Own)
    let mut result = 0;
    let file = File::open("resources/input.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();

        let len = line.len() / 2;

        let first = &line[0..len];
        let end = &line[len..];

        let shared_letters = same_letters(first, end);

        let value: i32;
        match shared_letters {
            Some(letters) => {
                value = letters.chars().map(|c| letter_value(c).unwrap()).sum();
            }
            None => value = 0,
        }

        result += value;
    }
    println!("Result: {}", result);

    // Part II (Group of 3 badges)
    let file = File::open("resources/input.txt").unwrap();
    let reader = BufReader::new(file);

    let badges = find_badges(reader).unwrap();
    let sum: i32 = badges.iter().map(|c| letter_value(*c).unwrap()).sum();
    println!("Badge sum: {}", sum);
}

fn same_letters(a: &str, b: &str) -> Option<String> {
    let mut result = Vec::new();

    let a_chars = a.chars();
    let b_chars = b.chars();

    for a_letter in a_chars {
        for b_letter in b_chars.clone() {
            if a_letter == b_letter {
                result.push(a_letter)
            }
        }
    }

    result.sort();
    result.dedup();

    if result.len() > 0 {
        let result_string: String = result.iter().collect();
        Some(result_string)
    } else {
        None
    }
}

fn letter_value(c: char) -> Result<i32, &'static str> {
    if !c.is_ascii_alphabetic() {
        return Err("Invalid letter");
    }

    let mut letter_value = c as i32;

    if c.is_uppercase() {
        letter_value -= 'A' as i32;
        letter_value += 27;
    } else if c.is_lowercase() {
        letter_value -= 'a' as i32;
        letter_value += 1;
    } else {
        return Err("Invalid Letter");
    }

    Ok(letter_value)
}

fn find_badges(reader: BufReader<File>) -> Result<Vec<char>, &'static str> {
    let mut lines = reader.lines();
    let mut group: Vec<String>;
    let mut group_badges: Vec<char> = Vec::new();

    loop {
        group = lines.by_ref().take(3).map(|s| s.unwrap()).collect();

        if group.is_empty() {
            break;
        }

        let a_b = same_letters(group[0].as_str(), group[1].as_str()).unwrap();
        let a_b_c = same_letters(a_b.as_str(), group[2].as_str()).unwrap();

        if a_b_c.len() == 1 {
            let c = a_b_c.chars().next().unwrap();
            group_badges.push(c);
        } else {
            return Err("Multiple badges for one group found");
        }
    }

    Ok(group_badges)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_same_letters() {
        let a = "AbC";
        let b = "abc";

        let result = same_letters(a, b);

        assert!(!result.is_none());
        assert_eq!("b".to_string(), result.unwrap());
    }

    #[test]
    fn test_same_letters_none() {
        let a = "AbC";
        let b = "aBc";

        let result = same_letters(a, b);

        assert!(result.is_none());
    }

    #[test]
    fn test_latter_value() {
        assert_eq!(1, letter_value('a').unwrap());
        assert_eq!(27, letter_value('A').unwrap());
        assert_eq!(3, letter_value('c').unwrap());
        assert_eq!(26, letter_value('z').unwrap());
        assert_eq!(52, letter_value('Z').unwrap());

        assert!(letter_value('6').is_err());
        assert!(letter_value('-').is_err());
    }
}
