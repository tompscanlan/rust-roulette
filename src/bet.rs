use crate::RouletteSpin;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Bet {
    pub amount: i32,
    pub bet_type: BetType,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum BetType {
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
    pub fn pays(&self, spin: &RouletteSpin) -> i32 {
        // dbg!(self, spin);
        match self.bet_type {
            BetType::Single(n) => {
                if *spin == RouletteSpin::Number(n) {
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
                        if n > &(street * 3) && n <= &(street * 3 + 3) {
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
                    if RouletteSpin::color(n) == "Red" {
                        self.amount + self.amount
                    } else {
                        0
                    }
                }
                _ => 0,
            },
            BetType::Black => match spin {
                RouletteSpin::Number(n) => {
                    if RouletteSpin::color(n) == "Black" {
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
