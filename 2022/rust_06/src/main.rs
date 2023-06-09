// https://adventofcode.com/2022/day/6

use std::{
    io::{self},
    process,
};

const START_OF_COMMUNICATION: usize = 4;
const START_OF_MESSAGE: usize = 14;

fn main() -> io::Result<()> {
    let input = include_str!("../resources/input.txt");

    if let Some(index) = find_marker(input, START_OF_COMMUNICATION) {
        println!("Packet found at index {}.", index);
    } else {
        eprintln!("Packet not found.");
        process::exit(1);
    }

    if let Some(index) = find_marker(input, START_OF_MESSAGE) {
        println!("Message found at index {}.", index);
    } else {
        eprintln!("Message not found.");
        process::exit(2);
    }

    Ok(())
}

fn find_marker(txt: &str, marker_len: usize) -> Option<i32> {
    let mut buffer: Vec<char> = Vec::new();

    for (count, character) in txt.chars().enumerate() {
        if let Some(index) = buffer.iter().position(|&e| e == character) {
            buffer.drain(0..=index);
        }

        buffer.push(character);

        if buffer.len() == marker_len {
            return Some((count + 1) as i32);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_1() {
        let txt = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let a = find_marker(txt, START_OF_COMMUNICATION).unwrap();
        assert_eq!(a, 5);
    }

    #[test]
    fn test_part_1_2() {
        let txt = "nppdvjthqldpwncqszvftbrmjlhg";
        let a = find_marker(txt, START_OF_COMMUNICATION).unwrap();
        assert_eq!(a, 6);
    }

    #[test]
    fn test_part_1_3() {
        let txt = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let a = find_marker(txt, START_OF_COMMUNICATION).unwrap();
        assert_eq!(a, 10);
    }

    #[test]
    fn test_part_1_4() {
        let txt = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let a = find_marker(txt, START_OF_COMMUNICATION).unwrap();
        assert_eq!(a, 11);
    }

    #[test]
    fn test_part_2_1() {
        let txt = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let a = find_marker(txt, START_OF_MESSAGE).unwrap();
        assert_eq!(a, 19);
    }

    #[test]
    fn test_part_2_2() {
        let txt = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let a = find_marker(txt, START_OF_MESSAGE).unwrap();
        assert_eq!(a, 23);
    }

    #[test]
    fn test_part_2_3() {
        let txt = "nppdvjthqldpwncqszvftbrmjlhg";
        let a = find_marker(txt, START_OF_MESSAGE).unwrap();
        assert_eq!(a, 23);
    }

    #[test]
    fn test_part_2_4() {
        let txt = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let a = find_marker(txt, START_OF_MESSAGE).unwrap();
        assert_eq!(a, 29);
    }

    #[test]
    fn test_part_2_5() {
        let txt = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let a = find_marker(txt, START_OF_MESSAGE).unwrap();
        assert_eq!(a, 26);
    }
}
