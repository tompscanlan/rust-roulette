use crate::{Bet, BetType, RouletteSpin};

use super::Strategy;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Cross {
}

impl Strategy for Cross {
    fn next_bet(&mut self, spins: Vec<RouletteSpin>, bets: Vec<Bet>) -> Bet {
      todo!()
      // how do we place more than one bet per spin?
      // this would be a street or dozen bet and a column bet
    }
}
