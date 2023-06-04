#[derive(Debug, PartialEq)]

pub enum GoalEnum {
    Win,
    Lose,
    Draw,
}

impl GoalEnum {
    pub fn from_symbol(symbol: char) -> Result<GoalEnum, &'static str> {
        let symbol = symbol.to_ascii_uppercase();
        match symbol {
            'X' => Ok(GoalEnum::Lose),
            'Y' => Ok(GoalEnum::Draw),
            'Z' => Ok(GoalEnum::Win),
            _ => Err("Unknown Symbol"),
        }
    }

    pub fn get_value(&self) -> i32 {
        match self {
            GoalEnum::Win => 6,
            GoalEnum::Draw => 3,
            GoalEnum::Lose => 0,
        }
    }
}
