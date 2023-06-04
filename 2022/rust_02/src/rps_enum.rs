#[derive(Debug, PartialEq)]
pub enum RPSEnum {
    Rock,
    Paper,
    Scissors,
}

impl RPSEnum {
    pub fn get_value(&self) -> i32 {
        match self {
            RPSEnum::Rock => 1,
            RPSEnum::Paper => 2,
            RPSEnum::Scissors => 3,
        }
    }

    pub fn from_symbol(symbol: char) -> Result<RPSEnum, &'static str> {
        let symbol = symbol.to_ascii_uppercase();
        match symbol {
            'A' | 'X' => Ok(RPSEnum::Rock),
            'B' | 'Y' => Ok(RPSEnum::Paper),
            'C' | 'Z' => Ok(RPSEnum::Scissors),
            _ => Err("Unknown Symbol"),
        }
    }
}
