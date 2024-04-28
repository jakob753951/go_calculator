use std::str::FromStr;
use crate::rank::Rank;

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Player {
    pub(crate) name: String,
    pub(crate) rank: Rank,
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


#[cfg(test)]
mod tests {
    use crate::player::*;

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
}