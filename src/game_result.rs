use std::str::FromStr;
use crate::game_result::GameResult::*;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum GameResult {
    White,
    Black,
    Draw
}

impl FromStr for GameResult {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (black, white) = s.split_once('-').ok_or(())?;
        let black = black.parse::<u8>().ok().ok_or(())?;
        let white = white.parse::<u8>().ok().ok_or(())?;
        if black > white {
            return Ok(Black);
        }
        if white > black {
            return Ok(White);
        }
        Ok(Draw)
    }
}

#[cfg(test)]
mod tests {
    use crate::game_result::*;

    #[test]
    fn parse_game_result_black_wins() {
        let test_input = "1-0";
        let result = test_input.parse::<GameResult>();
        assert_eq!(result, Ok(Black));
    }
    
    #[test]
    fn parse_game_result_white_wins() {
        let test_input = "0-1";
        let result = test_input.parse::<GameResult>();
        assert_eq!(result, Ok(White));
    }
    
    #[test]
    fn parse_game_result_draw() {
        let test_input = "0-0";
        let result = test_input.parse::<GameResult>();
        assert_eq!(result, Ok(Draw));
    }
}