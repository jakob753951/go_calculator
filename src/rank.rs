use std::str::FromStr;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Rank {
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


#[cfg(test)]
mod tests {
    use crate::rank::*;
    use crate::rank::Rank::*;

    #[test]
    fn parse_dan() {
        let test_rank = "8 Dan";
        let result = test_rank.parse::<Rank>();
        assert_eq!(result, Ok(Dan(8)));
    }

    #[test]
    fn parse_kyu() {
        let test_rank = "10 Kyu";
        let result = test_rank.parse::<Rank>();
        assert_eq!(result, Ok(Kyu(10)));
    }
}