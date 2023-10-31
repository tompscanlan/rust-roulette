pub mod doubleonloss;
use crate::{Bet, RouletteSpin};

pub(crate) trait Strategy {
    fn next_bet(&mut self, spins: Vec<RouletteSpin>, bets: Vec<Bet>) -> Bet;
}
