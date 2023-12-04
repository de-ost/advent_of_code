// https://adventofcode.com/2023/day/2

const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;

struct Round {
    red: i32,
    green: i32,
    blue: i32,
}

impl Default for Round {
    fn default() -> Self {
        Round {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

fn main() {
    let input = include_str!("../resources/input.txt");
    let result = result(input);
    println!("Result: {}", result);
}

fn result(input: &str) -> u32 {
    // Input data structure example:
    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    // Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    // Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    // Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    // Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green

    let mut result_counter = 0;

    for line in input.lines() {
        let mut game_possible = true;

        let (game, data) = line
            .split_once(":")
            .expect("Invalid input: Missing game number");

        let game_number = game // Format: "Game 1" -> "1"
            .split_once(" ")
            .expect("Invalid input: Invalid game format")
            .1
            .parse::<u32>()
            .expect("Invalid input: Failed to parse game number");

        for round in data.split(";") {
            let round_txt = round.split(","); // Format: "3 blue, 4 red" -> ["3 blue", "4 red"]
            let mut round = Round::default();

            for round_data in round_txt {
                let (count, color) = round_data // Format: "3 blue" -> ("3", "blue")
                    .trim()
                    .split_once(" ")
                    .expect("Invalid input: Invalid round format");

                let count = count
                    .parse::<i32>()
                    .expect("Invalid input: Failed to parse round count");

                match color {
                    "red" => round.red += count,
                    "green" => round.green += count,
                    "blue" => round.blue += count,
                    _ => panic!("Invalid input: Invalid color"),
                }
            }

            if round.blue > MAX_BLUE || round.green > MAX_GREEN || round.red > MAX_RED {
                game_possible = false;
                break;
            }
        }

        if game_possible {
            result_counter += game_number;
        }
    }

    result_counter
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test() {
        let result = result(TEST_INPUT);
        assert_eq!(result, 8);
    }
}
