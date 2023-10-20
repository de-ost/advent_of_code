use super::goal_enum::GoalEnum;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RPSEnum {
    Rock,
    Paper,
    Scissors,
}

impl RPSEnum {
    pub fn from_symbol(symbol: char) -> Result<RPSEnum, &'static str> {
        let symbol = symbol.to_ascii_uppercase();
        match symbol {
            'A' => Ok(RPSEnum::Rock),
            'B' => Ok(RPSEnum::Paper),
            'C' => Ok(RPSEnum::Scissors),
            _ => Err("Unknown Symbol"),
        }
    }

    pub fn get_value(&self) -> i32 {
        match self {
            RPSEnum::Rock => 1,
            RPSEnum::Paper => 2,
            RPSEnum::Scissors => 3,
        }
    }

    /// Checks if self won against other.
    pub fn won_against(&self, other: RPSEnum) -> GoalEnum {
        match (self, other) {
            (RPSEnum::Rock, RPSEnum::Paper) => GoalEnum::Lose,
            (RPSEnum::Rock, RPSEnum::Scissors) => GoalEnum::Win,
            (RPSEnum::Paper, RPSEnum::Rock) => GoalEnum::Win,
            (RPSEnum::Paper, RPSEnum::Scissors) => GoalEnum::Lose,
            (RPSEnum::Scissors, RPSEnum::Rock) => GoalEnum::Lose,
            (RPSEnum::Scissors, RPSEnum::Paper) => GoalEnum::Win,

            (RPSEnum::Rock, RPSEnum::Rock)
            | (RPSEnum::Paper, RPSEnum::Paper)
            | (RPSEnum::Scissors, RPSEnum::Scissors) => GoalEnum::Draw,
        }
    }

    pub fn calculate_turn(&self, goal: GoalEnum) -> Result<RPSEnum, &'static str> {
        let all_variants = vec![RPSEnum::Rock, RPSEnum::Paper, RPSEnum::Scissors];

        for variant in all_variants {
            let result: GoalEnum = variant.won_against(*self);
            
            if result == goal {
                return Ok(variant);
            }
        }

        Err("Could not calculate requested turn")
    }
}
