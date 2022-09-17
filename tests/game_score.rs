// use tenis_actions;
// Testing of GameScore Struct
// #[cfg(test)]
// mod tests {
//     use super::*;
// mod tenis_actions;
use tennis_score::tenis_actions::{Game, Player, Score, Stage};

#[test]
fn initialize() {
    assert_eq!(
        Game::new(),
        Game {
            score: Score {
                home: 0,
                oponent: 0,
            },
            stage: Stage::Normal,
            sets: [(0, 0, false); 3]
        }
    )
}
#[test]
fn add_point_test() {
    let game = Game {
        score: Score {
            home: 0,
            oponent: 0,
        },
        stage: Stage::Normal,
        sets: [(0, 0, false); 3],
    };
    assert_eq!(
        game.add_point(Player::Home).add_point(Player::Oponent),
        Game {
            score: Score {
                home: 15,
                oponent: 15,
            },
            stage: Stage::Normal,
            sets: [(0, 0, false); 3],
        }
    );
}
#[test]
fn make_deuce() {
    let game = Game {
        score: Score {
            home: 30,
            oponent: 40,
        },
        stage: Stage::Normal,
        sets: [(0, 0, false); 3],
    };
    assert_eq!(
        game.add_point(Player::Home),
        Game {
            score: Score {
                home: 0,
                oponent: 0,
            },
            stage: Stage::Deuce,
            sets: [(0, 0, false); 3],
        }
    );
}
#[test]
fn solve_deuce() {
    let game = Game {
        score: Score {
            home: 1,
            oponent: 0,
        },
        stage: Stage::Deuce,
        sets: [(0, 0, false); 3],
    };
    assert_eq!(
        game.add_point(Player::Home),
        Game {
            score: Score {
                home: 0,
                oponent: 0,
            },
            stage: Stage::Normal,
            sets: [(1, 0, false), (0, 0, false), (0, 0, false)],
        }
    );
}
#[test]
fn first_game_win() {
    let game = Game {
        score: Score {
            home: 40,
            oponent: 30,
        },
        stage: Stage::Normal,
        sets: [(0, 0, false); 3],
    };
    assert_eq!(
        game.add_point(Player::Home),
        Game {
            score: Score {
                home: 0,
                oponent: 0,
            },
            stage: Stage::Normal,
            sets: [(1, 0, false), (0, 0, false), (0, 0, false)],
        }
    );
    let game2 = Game {
        score: Score {
            home: 30,
            oponent: 40,
        },
        stage: Stage::Normal,
        sets: [(0, 0, false); 3],
    };
    assert_eq!(
        game2.add_point(Player::Oponent),
        Game {
            score: Score {
                home: 0,
                oponent: 0,
            },
            stage: Stage::Normal,
            sets: [(0, 1, false), (0, 0, false), (0, 0, false)],
        }
    );
}
#[test]
fn check_first_game_turns_true() {
    let game = Game {
        score: Score {
            home: 40,
            oponent: 30,
        },
        stage: Stage::Normal,
        sets: [(5, 4, false), (0, 0, false), (0, 0, false)],
    };
    assert_eq!(
        game.add_point(Player::Home),
        Game {
            score: Score {
                home: 0,
                oponent: 0,
            },
            stage: Stage::Normal,
            sets: [(6, 4, true), (0, 0, false), (0, 0, false)],
        }
    );
}

#[test]
fn second_game_win() {
    let game = Game {
        score: Score {
            home: 40,
            oponent: 30,
        },
        stage: Stage::Normal,
        sets: [(6, 4, true), (0, 0, false), (0, 0, false)],
    };
    assert_eq!(
        game.add_point(Player::Home),
        Game {
            score: Score {
                home: 0,
                oponent: 0,
            },
            stage: Stage::Normal,
            sets: [(6, 4, true), (1, 0, false), (0, 0, false)],
        }
    );
}

#[test]
fn check_second_game_win_true() {
    let game = Game {
        score: Score {
            home: 40,
            oponent: 30,
        },
        stage: Stage::Normal,
        sets: [(6, 4, true), (5, 4, false), (0, 0, false)],
    };
    assert_eq!(
        game.add_point(Player::Home),
        Game {
            score: Score {
                home: 0,
                oponent: 0,
            },
            stage: Stage::Normal,
            sets: [(6, 4, true), (6, 4, true), (0, 0, false)],
        }
    );
}
#[test]
fn check_deuce_and_then_tie_break() {
    let game = Game {
        score: Score {
            home: 30,
            oponent: 30,
        },
        stage: Stage::Normal,
        sets: [(6, 5, false), (0, 0, false), (0, 0, false)],
    };
    assert_eq!(
        game.add_point(Player::Home)
            .add_point(Player::Oponent)
            .add_point(Player::Oponent)
            .add_point(Player::Oponent),
        Game {
            score: Score {
                home: 0,
                oponent: 0,
            },
            stage: Stage::TieBreak,
            sets: [(6, 6, false), (0, 0, false), (0, 0, false)],
        }
    );
}
#[test]
fn solve_tie_break() {
    let game = Game {
        score: Score {
            home: 6,
            oponent: 5,
        },
        stage: Stage::TieBreak,
        sets: [(6, 6, false), (0, 0, false), (0, 0, false)],
    };
    assert_eq!(
        game.add_point(Player::Home),
        Game {
            score: Score {
                home: 0,
                oponent: 0,
            },
            stage: Stage::Normal,
            sets: [(7, 6, true), (0, 0, false), (0, 0, false)],
        }
    );
    let game2 = Game {
        score: Score {
            home: 6,
            oponent: 5,
        },
        stage: Stage::TieBreak,
        sets: [(6, 6, false), (0, 0, false), (0, 0, false)],
    };
    assert_eq!(
        game2
            .add_point(Player::Oponent)
            .add_point(Player::Oponent)
            .add_point(Player::Oponent)
            .add_point(Player::Oponent),
        Game {
            score: Score {
                home: 0,
                oponent: 15,
            },
            stage: Stage::Normal,
            sets: [(6, 7, true), (0, 0, false), (0, 0, false)],
        }
    );
}

#[test]
fn long_tie_break_in_second_set() {
    // HOME WINS
    let game = Game {
        score: Score {
            home: 6,
            oponent: 5,
        },
        stage: Stage::TieBreak,
        sets: [(7, 6, true), (6, 6, false), (0, 0, false)],
    };
    assert_eq!(
        game.add_point(Player::Home),
        Game {
            score: Score {
                home: 0,
                oponent: 0,
            },
            stage: Stage::Normal,
            sets: [(7, 6, true), (7, 6, true), (0, 0, false)],
        }
    );

    // OPONENT WINS
    let game2 = Game {
        score: Score {
            home: 6,
            oponent: 5,
        },
        stage: Stage::TieBreak,
        sets: [(6, 7, true), (6, 6, false), (0, 0, false)],
    };
    assert_eq!(
        game2
            .add_point(Player::Oponent)
            .add_point(Player::Oponent)
            .add_point(Player::Oponent)
            .add_point(Player::Oponent),
        Game {
            score: Score {
                home: 0,
                oponent: 15,
            },
            stage: Stage::Normal,
            sets: [(6, 7, true), (6, 7, true), (0, 0, false)],
        }
    );
}

#[test]
fn third_set_starts_and_oponent_wins_first_deuce() {
    let game = Game {
        score: Score {
            home: 40,
            oponent: 0,
        },
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
        Game {
            score: Score {
                home: 0,
                oponent: 0,
            },
            stage: Stage::Normal,
            sets: [(7, 6, true), (7, 5, true), (0, 1, false)],
        }
    );
}
// }
