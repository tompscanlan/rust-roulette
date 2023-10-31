mod bet;
mod main_test;
mod strategy;

mod spin;
use bet::{Bet, BetType};
use spin::RouletteSpin;
use strategy::doubleonloss::DoubleOnLoss;
use strategy::Strategy;

struct RouletteSession<'a> {
    bets: Vec<Bet>,
    spins: Vec<RouletteSpin>,
    strategy: Box<dyn Strategy + 'a>,
    start_amount: i32,
    amount: i32,
}

fn main() {
    let mut session = RouletteSession {
        bets: vec![],
        spins: vec![],
        start_amount: 100,
        amount: 100,
        strategy: Box::new(DoubleOnLoss {}),
    };

    for _ in 0..100000 {
        println!("Cash: {}", session.amount);
        let s = &mut session.strategy;

        let bet = s.next_bet(session.spins.clone(), session.bets.clone());
        println!("Placing bet: {:?}", bet);
        session.bets.push(bet);
        session.amount -= bet.amount;

        let spin = RouletteSpin::spin();
        session.spins.push(spin);
        println!("Spin turned up {}", spin);

        let pay_out = bet.pays(&spin);
        session.amount += pay_out;
        println!("Pay out: {}", pay_out);

        if session.amount <= 0 {
            println!("You lost it all!");
            print!("Spins: {:?}", session.spins.len());
            break;
        }

        // if session.amount >= session.start_amount * 2 {
        //     println!("You doubled your money!");
        // }
    }
}
