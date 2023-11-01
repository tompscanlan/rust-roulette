use crate::{bet::BetType, Bet, RouletteSpin};

use super::Strategy;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Cross {}

impl Strategy for Cross {
    fn next_bet(&mut self, spins: Vec<RouletteSpin>, bets: Vec<Vec<Bet>>) -> Vec<Bet> {
        return vec![
            Bet {
                amount: 1,
                bet_type: BetType::Dozen(1),
            },
            Bet {
                amount: 1,
                bet_type: BetType::Black,
            },
        ];
    }
}
