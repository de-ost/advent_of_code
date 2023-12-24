use std::collections::HashMap;

pub fn result(input: &str) -> u32 {
    // Input data structure example:
    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    // Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    // Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    // Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    // Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green

    let mut result_sum = 0;

    for line in input.lines() {
        let (_game, data) = line
            .split_once(":")
            .expect("Invalid input: Missing game number");

        let mut minimum_of_colors: HashMap<&str, u32> = HashMap::new();
        minimum_of_colors.insert("red", 0);
        minimum_of_colors.insert("green", 0);
        minimum_of_colors.insert("blue", 0);

        for round in data.split(";") {
            let round_txt = round.split(","); // Format: "3 blue, 4 red" -> ["3 blue", "4 red"]

            for round_data in round_txt {
                let (count, color) = round_data // Format: "3 blue" -> ("3", "blue")
                    .trim()
                    .split_once(" ")
                    .expect("Invalid input: Invalid round format");

                let count = count
                    .parse::<u32>()
                    .expect("Invalid input: Failed to parse round count");

                match color {
                    "red" | "green" | "blue" => {
                        let min_color = minimum_of_colors.get_mut(color).expect("Invalid color");
                        if count > *min_color {
                            *min_color = count;
                        }
                    }
                    _ => panic!("Invalid input: Invalid color"),
                }
            }
        }
        result_sum += minimum_of_colors.values().product::<u32>();
    }

    result_sum
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
        assert_eq!(result, 2286);
    }
}
