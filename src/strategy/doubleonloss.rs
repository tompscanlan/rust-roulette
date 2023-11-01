use crate::{Bet, BetType, RouletteSpin};

use super::Strategy;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct DoubleOnLoss {
}

impl Strategy for DoubleOnLoss {
    fn next_bet(&mut self, spins: Vec<RouletteSpin>, bets: Vec<Bet>) -> Bet {
        if bets.is_empty() || spins.is_empty() {
            return Bet {
                amount: 1,
                bet_type: BetType::Black,
            };
        }
        if bets.last().unwrap().pays(spins.last().unwrap()) == 0 {
            Bet {
                amount: bets.last().unwrap().amount * 2,
                bet_type: BetType::Black,
            }
        } else {
            Bet {
                amount: 1,
                bet_type: BetType::Black,
            }
        }
    }
}
