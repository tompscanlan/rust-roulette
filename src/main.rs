use std::fmt;

use rand::{self, Rng};

#[derive(Debug, PartialEq)]
enum RouletteSpin {
    DoubleZero,
    Zero,
    Number(i32),
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
        if n < &1 {
            "None".to_string()
        } else if n % 2 == 0 {
            "Black".to_string()
        } else {
            "Red".to_string()
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
}
