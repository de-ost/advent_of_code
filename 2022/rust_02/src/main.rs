mod rps_enum;
use rps_enum::RPSEnum;

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
                let player_turn = RPSEnum::from_symbol(symbol_player).unwrap();

                let value = player_turn.get_value();
                let win = winner(player_turn, enemy_turn);

                result += win + value;
            }
            _ => (),
        }
    }

    println!("Total result: {:?}", result);
}

/// Checks if a won against b.
///
/// Possible returns:
///     - 0: if a lost
///     - 3: if it's a draw
///     - 6: if a won
fn winner(a: RPSEnum, b: RPSEnum) -> i32 {
    match (a, b) {
        (RPSEnum::Rock, RPSEnum::Paper) => 0,
        (RPSEnum::Rock, RPSEnum::Scissors) => 6,
        (RPSEnum::Paper, RPSEnum::Rock) => 6,
        (RPSEnum::Paper, RPSEnum::Scissors) => 0,
        (RPSEnum::Scissors, RPSEnum::Rock) => 0,
        (RPSEnum::Scissors, RPSEnum::Paper) => 6,

        (RPSEnum::Rock, RPSEnum::Rock)
        | (RPSEnum::Paper, RPSEnum::Paper)
        | (RPSEnum::Scissors, RPSEnum::Scissors) => 3,
    }
}
