use std::fmt;

use rand::{self, Rng};

#[derive(Debug, PartialEq)]
enum RouletteSpin {
    DoubleZero,
    Zero,
    Number(i32),
}

#[derive(Debug, PartialEq)]
struct Bets {
    bets: Vec<Bet>,
}

#[derive(Debug, PartialEq)]
struct Bet {
    amount: i32,
    bet_type: BetType,
}

#[derive(Debug, PartialEq)]
enum BetType {
    Single(i32),
    Split(i32, i32),
    Street(i32),
    DoubleStreet,
    Basket,
    FirstFour,
    TopLine,
    Corner,
    Low,
    High,
    Red,
    Black,
    Even,
    Odd,
    Dozen,
    Column,
    Snake,
}

impl Bet {
    fn pays(&self, spin: RouletteSpin) -> i32 {
    //     return match spin {
    //         RouletteSpin::Number(n) => self.amount * 35,
    //         RouletteSpin::Street(n) => self.amount * 11,
    //         _ => 0,
    //     };
    // }
        match self.bet_type {
            BetType::Single(n) => {
                if spin == RouletteSpin::Number(n) {
                    35 * self.amount
                } else {
                    0
                }
            },
            BetType::Street(n) => {
                if spin == RouletteSpin::Number(n) {
                    11 * self.amount
                } else {
                    0
                }
            },

            _ => 0,
        }
    }
}

impl RouletteSpin {
    fn spin() -> RouletteSpin {
        let n = rand::thread_rng().gen_range(-1..36);
        if n == -1 {
            RouletteSpin::DoubleZero
        } else if n == 0 {
            RouletteSpin::Zero
        } else {
            RouletteSpin::Number(n)
        }
    }

    fn color(n: &i32) -> String {
        return match n {
            1 => "Red".to_string(),
            2 => "Black".to_string(),
            3 => "Red".to_string(),
            4 => "Black".to_string(),
            5 => "Red".to_string(),
            6 => "Black".to_string(),
            7 => "Red".to_string(),
            8 => "Black".to_string(),
            9 => "Red".to_string(),
            10 => "Black".to_string(),
            11 => "Black".to_string(),
            12 => "Red".to_string(),
            13 => "Black".to_string(),
            14 => "Red".to_string(),
            15 => "Black".to_string(),
            16 => "Red".to_string(),
            17 => "Black".to_string(),
            18 => "Red".to_string(),
            19 => "Red".to_string(),
            20 => "Black".to_string(),
            21 => "Red".to_string(),
            22 => "Black".to_string(),
            23 => "Red".to_string(),
            24 => "Black".to_string(),
            25 => "Red".to_string(),
            26 => "Black".to_string(),
            27 => "Red".to_string(),
            28 => "Black".to_string(),
            29 => "Black".to_string(),
            30 => "Red".to_string(),
            31 => "Black".to_string(),
            32 => "Red".to_string(),
            33 => "Black".to_string(),
            34 => "Red".to_string(),
            35 => "Black".to_string(),
            36 => "Red".to_string(),
            _ => "Green".to_string(),
        };
    }
}
impl fmt::Display for RouletteSpin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RouletteSpin::Number(n) => write!(f, "{} {}", RouletteSpin::color(n), n),
            RouletteSpin::DoubleZero => write!(f, "00"),
            RouletteSpin::Zero => write!(f, "0"),
        }
    }
}

fn main() {
    println!("Hello, world: {}", RouletteSpin::spin());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spin_to_string() {
        let spin = RouletteSpin::Number(1);
        assert_eq!(spin.to_string(), "Red 1");

        let spin = RouletteSpin::Number(2);
        assert_eq!(spin.to_string(), "Black 2");

        let spin = RouletteSpin::DoubleZero;
        assert_eq!(spin.to_string(), "00");

        let spin = RouletteSpin::Zero;
        assert_eq!(spin.to_string(), "0");
    }

    #[test]
    fn bet() {
        let bet = Bet {
            amount: 1,
            bet_type: BetType::Single(1),
        };
        assert_eq!(bet.amount, 1);
        assert_eq!(bet.bet_type, BetType::Single(1));
    }

    #[test]
    fn single_bet_pays() {
        let bet = Bet {
            amount: 1,
            bet_type: BetType::Single(35),
        };
        let spin = RouletteSpin::Number(35);

        assert_eq!(bet.pays(spin), 35 * bet.amount);
    }

    #[test]
    fn street_bet_pays() {
        let bet = Bet {
            amount: 1,
            bet_type: BetType::Street(7),
        };

        let spin = RouletteSpin::Number(10);
        assert_eq!(bet.pays(spin), 0);

        let spin = RouletteSpin::Number(7);
        assert_eq!(bet.pays(spin), 11 * bet.amount);
    }
}
