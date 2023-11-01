use crate::{Bet, BetType, RouletteSpin};

use super::Strategy;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct DoubleOnLoss {}

impl Strategy for DoubleOnLoss {
    fn next_bet(&mut self, spins: Vec<RouletteSpin>, bets: Vec<Vec<Bet>>) -> Vec<Bet> {
        let mut next_bet = vec![];

        // When no prior bets, set initial bets
        if bets.is_empty() || spins.is_empty() {
            next_bet.push(Bet {
                amount: 1,
                bet_type: BetType::Black,
            });
            next_bet.push(Bet {
                amount: 1,
                bet_type: BetType::Odd,
            });
        } else {
            // Double last bet if it lost
            for bet in bets.last().unwrap() {
                if bet.pays(spins.last().unwrap()) == 0 {
                    next_bet.push(Bet {
                        amount: bet.amount * 2,
                        bet_type: bet.bet_type,
                    });
                } else {
                    next_bet.push(Bet {
                        amount: 1,
                        bet_type: bet.bet_type,
                    });
                }
            }
        }
        return next_bet;
    }
}
