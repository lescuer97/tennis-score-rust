// use tenis_actions;
// Testing of GameScore Struct
// #[cfg(test)]
// mod tests {
//     use super::*;
// mod tenis_actions;
use tennis_score::tenis_actions::{GameScore, Player, Stage};

#[test]
fn initialize() {
    assert_eq!(
        GameScore::new(),
        GameScore {
            score: [(Player::Home, 0), (Player::Oponent, 0)],
            stage: Stage::Normal,
            sets: [(0, 0, false); 3]
        }
    )
}
#[test]
fn add_point_test() {
    let game = GameScore {
        score: [(Player::Home, 0), (Player::Oponent, 0)],
        stage: Stage::Normal,
        sets: [(0, 0, false); 3],
    };
    assert_eq!(
        game.add_point(Player::Home).add_point(Player::Oponent),
        GameScore {
            score: [(Player::Home, 15), (Player::Oponent, 15)],
            stage: Stage::Normal,
            sets: [(0, 0, false); 3],
        }
    );
}
#[test]
fn make_deuce() {
    let game = GameScore {
        score: [(Player::Home, 30), (Player::Oponent, 40)],
        stage: Stage::Normal,
        sets: [(0, 0, false); 3],
    };
    assert_eq!(
        game.add_point(Player::Home),
        GameScore {
            score: [(Player::Home, 0), (Player::Oponent, 0)],
            stage: Stage::Deuce,
            sets: [(0, 0, false); 3],
        }
    );
}
#[test]
fn solve_deuce() {
    let game = GameScore {
        score: [(Player::Home, 1), (Player::Oponent, 0)],
        stage: Stage::Deuce,
        sets: [(0, 0, false); 3],
    };
    assert_eq!(
        game.add_point(Player::Home),
        GameScore {
            score: [(Player::Home, 0), (Player::Oponent, 0)],
            stage: Stage::Normal,
            sets: [(1, 0, false), (0, 0, false), (0, 0, false)],
        }
    );
}
#[test]
fn first_game_win() {
    let game = GameScore {
        score: [(Player::Home, 40), (Player::Oponent, 30)],
        stage: Stage::Normal,
        sets: [(0, 0, false); 3],
    };
    assert_eq!(
        game.add_point(Player::Home),
        GameScore {
            score: [(Player::Home, 0), (Player::Oponent, 0)],
            stage: Stage::Normal,
            sets: [(1, 0, false), (0, 0, false), (0, 0, false)],
        }
    );
    let game2 = GameScore {
        score: [(Player::Home, 30), (Player::Oponent, 40)],
        stage: Stage::Normal,
        sets: [(0, 0, false); 3],
    };
    assert_eq!(
        game2.add_point(Player::Oponent),
        GameScore {
            score: [(Player::Home, 0), (Player::Oponent, 0)],
            stage: Stage::Normal,
            sets: [(0, 1, false), (0, 0, false), (0, 0, false)],
        }
    );
}
#[test]
fn check_first_game_turns_true() {
    let game = GameScore {
        score: [(Player::Home, 40), (Player::Oponent, 30)],
        stage: Stage::Normal,
        sets: [(5, 4, false), (0, 0, false), (0, 0, false)],
    };
    assert_eq!(
        game.add_point(Player::Home),
        GameScore {
            score: [(Player::Home, 0), (Player::Oponent, 0)],
            stage: Stage::Normal,
            sets: [(6, 4, true), (0, 0, false), (0, 0, false)],
        }
    );
}

#[test]
fn second_game_win() {
    let game = GameScore {
        score: [(Player::Home, 40), (Player::Oponent, 30)],
        stage: Stage::Normal,
        sets: [(6, 4, true), (0, 0, false), (0, 0, false)],
    };
    assert_eq!(
        game.add_point(Player::Home),
        GameScore {
            score: [(Player::Home, 0), (Player::Oponent, 0)],
            stage: Stage::Normal,
            sets: [(6, 4, true), (1, 0, false), (0, 0, false)],
        }
    );
}

#[test]
fn check_second_game_win_true() {
    let game = GameScore {
        score: [(Player::Home, 40), (Player::Oponent, 30)],
        stage: Stage::Normal,
        sets: [(6, 4, true), (5, 4, false), (0, 0, false)],
    };
    assert_eq!(
        game.add_point(Player::Home),
        GameScore {
            score: [(Player::Home, 0), (Player::Oponent, 0)],
            stage: Stage::Normal,
            sets: [(6, 4, true), (6, 4, true), (0, 0, false)],
        }
    );
}
#[test]
fn check_deuce_and_then_tie_break() {
    let game = GameScore {
        score: [(Player::Home, 30), (Player::Oponent, 30)],
        stage: Stage::Normal,
        sets: [(6, 5, false), (0, 0, false), (0, 0, false)],
    };
    assert_eq!(
        game.add_point(Player::Home)
            .add_point(Player::Oponent)
            .add_point(Player::Oponent)
            .add_point(Player::Oponent),
        GameScore {
            score: [(Player::Home, 0), (Player::Oponent, 0)],
            stage: Stage::TieBreak,
            sets: [(6, 6, false), (0, 0, false), (0, 0, false)],
        }
    );
}
#[test]
fn solve_tie_break() {
    let game = GameScore {
        score: [(Player::Home, 6), (Player::Oponent, 5)],
        stage: Stage::TieBreak,
        sets: [(6, 6, false), (0, 0, false), (0, 0, false)],
    };
    assert_eq!(
        game.add_point(Player::Home),
        GameScore {
            score: [(Player::Home, 0), (Player::Oponent, 0)],
            stage: Stage::Normal,
            sets: [(7, 6, true), (0, 0, false), (0, 0, false)],
        }
    );
    let game2 = GameScore {
        score: [(Player::Home, 6), (Player::Oponent, 5)],
        stage: Stage::TieBreak,
        sets: [(6, 6, false), (0, 0, false), (0, 0, false)],
    };
    assert_eq!(
        game2
            .add_point(Player::Oponent)
            .add_point(Player::Oponent)
            .add_point(Player::Oponent)
            .add_point(Player::Oponent),
        GameScore {
            score: [(Player::Home, 0), (Player::Oponent, 15)],
            stage: Stage::Normal,
            sets: [(6, 7, true), (0, 0, false), (0, 0, false)],
        }
    );
}

#[test]
fn long_tie_break_in_second_set() {
    // HOME WINS
    let game = GameScore {
        score: [(Player::Home, 6), (Player::Oponent, 5)],
        stage: Stage::TieBreak,
        sets: [(7, 6, true), (6, 6, false), (0, 0, false)],
    };
    assert_eq!(
        game.add_point(Player::Home),
        GameScore {
            score: [(Player::Home, 0), (Player::Oponent, 0)],
            stage: Stage::Normal,
            sets: [(7, 6, true), (7, 6, true), (0, 0, false)],
        }
    );

    // OPONENT WINS
    let game2 = GameScore {
        score: [(Player::Home, 6), (Player::Oponent, 5)],
        stage: Stage::TieBreak,
        sets: [(6, 7, true), (6, 6, false), (0, 0, false)],
    };
    assert_eq!(
        game2
            .add_point(Player::Oponent)
            .add_point(Player::Oponent)
            .add_point(Player::Oponent)
            .add_point(Player::Oponent),
        GameScore {
            score: [(Player::Home, 0), (Player::Oponent, 15)],
            stage: Stage::Normal,
            sets: [(6, 7, true), (6, 7, true), (0, 0, false)],
        }
    );
}

#[test]
fn third_set_starts_and_oponent_wins_first_deuce() {
    let game = GameScore {
        score: [(Player::Home, 40), (Player::Oponent, 0)],
        stage: Stage::Normal,
        sets: [(7, 6, true), (6, 5, false), (0, 0, false)],
    };
    assert_eq!(
        game.add_point(Player::Home)
            .add_point(Player::Home)
            .add_point(Player::Oponent)
            .add_point(Player::Home)
            .add_point(Player::Oponent)
            .add_point(Player::Home)
            .add_point(Player::Oponent)
            .add_point(Player::Home)
            .add_point(Player::Oponent)
            .add_point(Player::Oponent)
            .add_point(Player::Oponent),
        GameScore {
            score: [(Player::Home, 0), (Player::Oponent, 0)],
            stage: Stage::Normal,
            sets: [(7, 6, true), (7, 5, true), (0, 1, false)],
        }
    );
}
// }
