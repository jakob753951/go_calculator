use std::str::FromStr;
use regex::Regex;
use crate::game_result::GameResult;
use crate::player::Player;

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Game {
    pub board: String,
    pub black: Player,
    pub white: Player,
    pub result: GameResult,
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex_pattern = r" +(?<board>\d+) +(?<black>\w+ \(\d+ (Dan|Kyu)\)) +- (?<white>\w+ \(\d+ (Dan|Kyu)\)) +(?<result>\d-\d) *(((?<stones>\d) stones, )?(?<komi>½) komi)?";
        let re = Regex::new(regex_pattern).unwrap();
        let groups = re.captures(s).unwrap();

        let board = groups.name("board").ok_or(())?;
        let black = groups.name("black").ok_or(())?;
        let white = groups.name("white").ok_or(())?;
        let result = groups.name("result").ok_or(())?;

        let black: Player = black.as_str().parse::<Player>()?;
        let white: Player = white.as_str().parse::<Player>()?;
        let result: GameResult = result.as_str().parse::<GameResult>()?;

        Ok(Game {
            board: board.as_str().to_string(),
            black,
            white,
            result,
        })
    }
}


#[cfg(test)]
mod tests {
    use crate::game::*;

    #[test]
    fn parse_game() {
        let test_row = "   1   Player_83 (8 Dan)                          - Player_98 (9 Dan)                          0-1                              ";
        let result = test_row.parse::<Game>();
        let expected = Ok(Game {
            board: "1".to_string(),
            black: "Player_83 (8 Dan)".parse().unwrap(),
            white: "Player_98 (9 Dan)".parse().unwrap(),
            result: "0-1".parse().unwrap(),
        });
        assert_eq!(result, expected);
    }
}