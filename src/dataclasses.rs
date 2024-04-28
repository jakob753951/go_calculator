use std::str::FromStr;
use regex::Regex;

#[derive(PartialEq)]
#[derive(Debug)]
enum Rank {
    Kyu(u8),
    Dan(u8),
}

impl FromStr for Rank {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (number, rank_type) = s.split_once(' ').ok_or(())?;
        let number = number.parse::<u8>().ok().ok_or(())?;

        match rank_type.to_lowercase().as_str() {
            "kyu" => Ok(Rank::Kyu(number)),
            "dan" => Ok(Rank::Dan(number)),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
struct Player {
    name: String,
    rank: Rank,
}

impl FromStr for Player {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (player_name, rank) = s.split_once(' ').ok_or(())?;
        let rank = rank.trim_matches(&['(', ')']);
        Ok(Player {
            name: player_name.to_string(),
            rank: rank.parse()?,
        })
    }
}


#[derive(PartialEq)]
#[derive(Debug)]
enum GameResult {
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
            return Ok(GameResult::Black);
        }
        if white > black {
            return Ok(GameResult::White);
        }
        Ok(GameResult::Draw)
    }
}


#[derive(PartialEq)]
#[derive(Debug)]
struct Game {
    board: String,
    black: Player,
    white: Player,
    result: GameResult,
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
    use crate::dataclasses::*;

    #[test]
    fn parse_rank_dan() {
        let test_rank = "8 Dan";
        let result = test_rank.parse::<Rank>();
        let expected = Ok(Rank::Dan(8));
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_rank_kyu() {
        let test_rank = "10 Kyu";
        let result = test_rank.parse::<Rank>();
        let expected = Ok(Rank::Kyu(10));
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_player() {
        let test_player = "Player_91 (3 Dan)";
        let result = test_player.parse::<Player>();
        let expected = Ok(Player {
            name: "Player_91".to_string(),
            rank: Rank::Dan(3),
        });
        assert_eq!(result, expected);
    }


    #[test]
    fn parse_game() {
        let test_row = "   1   Player_83 (8 Dan)                          - Player_98 (9 Dan)                          0-1                              ";
        let result = test_row.parse::<Game>();
        let expected = Ok(Game {
            board: "1".to_string(),
            black: Player { name: "Player_83".to_string(), rank: Rank::Dan(8) },
            white: Player { name: "Player_98".to_string(), rank: Rank::Dan(9) },
            result: GameResult::White,
        });
        assert_eq!(result, expected);
    }
}