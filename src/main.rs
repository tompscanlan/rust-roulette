use rand::{self, Rng};
use std::fmt;

mod main_test;

#[derive(Debug, PartialEq, Copy, Clone)]
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

#[allow(dead_code)]
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
    #[allow(dead_code)]
    fn pays(&self, spin: RouletteSpin) -> i32 {
        // dbg!(self, spin);
        match self.bet_type {
            BetType::Single(n) => {
                if spin == RouletteSpin::Number(n) {
                    35 * self.amount
                } else {
                    0
                }
            }
            BetType::Street(street) => {
                assert!(
                    (0..=11).contains(&street),
                    "Street must be between 0 and 11"
                );
                match spin {
                    RouletteSpin::Number(n) => {
                        // dbg!(n, street * 3 + 1, street * 3 + 3);
                        if n > street * 3 && n <= street * 3 + 3 {
                            11 * self.amount
                        } else {
                            0
                        }
                    }
                    _ => 0,
                }
            }
            BetType::Red => match spin {
                RouletteSpin::Number(n) => {
                    if RouletteSpin::color(&n) == "Red" {
                        self.amount
                    } else {
                        0
                    }
                }
                _ => 0,
            },
            BetType::Black => match spin {
                RouletteSpin::Number(n) => {
                    if RouletteSpin::color(&n) == "Black" {
                        self.amount
                    } else {
                        0
                    }
                }
                _ => 0,
            },

            _ => {
                print!("Not implemented");
                0
            }
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
        match n {
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
        }
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
