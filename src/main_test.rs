#[allow(unused_imports)]
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

    assert_eq!(bet.pays(&spin), 35 * bet.amount + bet.amount);
}

#[test]
fn street_bet_pays() {
    let bet = Bet {
        amount: 1,
        bet_type: BetType::Street(2),
    };

    let spin = RouletteSpin::Number(10);
    assert_eq!(bet.pays(&spin), 0);

    let spin = RouletteSpin::Number(7);
    assert_eq!(bet.pays(&spin), 11 * bet.amount + bet.amount);

    let spin = RouletteSpin::Number(4);
    assert_eq!(bet.pays(&spin), 0);

    let bet = Bet {
        amount: 1,
        bet_type: BetType::Street(11),
    };
    let spin = RouletteSpin::Number(34);
    assert_eq!(bet.pays(&spin), 11 * bet.amount + bet.amount);

    let spin = RouletteSpin::Number(33);
    assert_eq!(bet.pays(&spin), 0);
}

#[test]
fn red_bet_pays() {
    let bets = vec![
        Bet {
            amount: 1,
            bet_type: BetType::Red,
        },
        Bet {
            amount: 20,
            bet_type: BetType::Red,
        },
    ];

    for bet in bets {
        let spin = RouletteSpin::Number(34);
        assert_eq!(bet.pays(&spin), bet.amount + bet.amount);

        let spin = RouletteSpin::Number(18);
        assert_eq!(bet.pays(&spin), bet.amount + bet.amount);

        let spin = RouletteSpin::Number(5);
        assert_eq!(bet.pays(&spin), bet.amount + bet.amount);

        let spin = RouletteSpin::DoubleZero;
        assert_eq!(bet.pays(&spin), 0);

        let spin = RouletteSpin::Zero;
        assert_eq!(bet.pays(&spin), 0);

        let spin = RouletteSpin::Number(2);
        assert_eq!(bet.pays(&spin), 0);
    }
}

#[test]
fn black_bet_pays() {
    let bets = vec![
        Bet {
            amount: 1,
            bet_type: BetType::Black,
        },
        Bet {
            amount: 20,
            bet_type: BetType::Black,
        },
    ];

    for bet in bets {
        let spin = RouletteSpin::Number(34);
        assert_eq!(bet.pays(&spin), 0);

        let spin = RouletteSpin::Number(18);
        assert_eq!(bet.pays(&spin), 0);

        let spin = RouletteSpin::Number(5);
        assert_eq!(bet.pays(&spin), 0);

        let spin = RouletteSpin::DoubleZero;
        assert_eq!(bet.pays(&spin), 0);

        let spin = RouletteSpin::Zero;
        assert_eq!(bet.pays(&spin), 0);

        let spin = RouletteSpin::Number(2);
        assert_eq!(bet.pays(&spin), bet.amount + bet.amount);
    }
}

#[test]
fn test_a_round() {
    let bets = vec![
        Bet {
            amount: 1,
            bet_type: BetType::Red,
        },
        Bet {
            amount: 20,
            bet_type: BetType::Black,
        },
        Bet {
            amount: 1,
            bet_type: BetType::Single(14),
        },
    ];

    let spin = RouletteSpin::spin();

    dbg!(spin);

    for bet in bets {
        dbg!(bet.pays(&spin));
    }
}

#[test]
fn dozen_bet_pays() {
    let bet = Bet {
        amount: 1,
        bet_type: BetType::Dozen(1),
    };

    let spin = RouletteSpin::Number(1);
    assert_eq!(bet.pays(&spin), 2 * bet.amount + bet.amount);

    let spin = RouletteSpin::Number(12);
    assert_eq!(bet.pays(&spin), 2 * bet.amount + bet.amount);

    let spin = RouletteSpin::Number(24);
    assert_eq!(bet.pays(&spin), 0);

    let bet = Bet {
        amount: 1,
        bet_type: BetType::Dozen(2),
    };

    let spin = RouletteSpin::Number(13);
    assert_eq!(bet.pays(&spin), 2 * bet.amount + bet.amount);

    let bet = Bet {
        amount: 1,
        bet_type: BetType::Dozen(3),
    };
    let spin = RouletteSpin::Number(23);
    assert_eq!(bet.pays(&spin), 0);

    let spin = RouletteSpin::Number(36);
    assert_eq!(bet.pays(&spin), 2 * bet.amount + bet.amount);

    let bet = Bet {
        amount: 1,
        bet_type: BetType::Dozen(4),
    };
    let spin = RouletteSpin::Number(24);
    assert_eq!(bet.pays(&spin), 0);
}
