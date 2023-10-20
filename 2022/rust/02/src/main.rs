mod rps_enum;
use rps_enum::RPSEnum;

mod goal_enum;
use goal_enum::GoalEnum;

use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut result = 0;

    let file = File::open("resources/input.txt").unwrap();
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        let mut symbols = line.split_whitespace();
        let enemy = symbols.next();
        let player = symbols.next();

        match (enemy, player) {
            (Some(enemy), Some(player)) => {
                let symbol_enemy = enemy.chars().next().unwrap();
                let enemy_turn = RPSEnum::from_symbol(symbol_enemy).unwrap();

                let symbol_player = player.chars().next().unwrap();
                let player_goal = GoalEnum::from_symbol(symbol_player).unwrap();

                let player_turn = enemy_turn.calculate_turn(player_goal).unwrap();
                let move_value = player_turn.get_value();

                let score = player_turn.won_against(enemy_turn).get_value();

                result += score + move_value;
            }
            _ => (),
        }
    }

    println!("Total result: {:?}", result);
}
