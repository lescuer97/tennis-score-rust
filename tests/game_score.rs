// use tenis_actions;
// Testing of GameScore Struct
// #[cfg(test)]
// mod tests {
//     use super::*;
// mod tenis_actions;
use tennis_score::tenis_actions::{Game, Player, Score, Sets, Stage};

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
            sets: [Sets {
                home: 0,
                oponent: 0,
                finished: false,
            }; 3]
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
        sets: [Sets {
            home: 0,
            oponent: 0,
            finished: false,
        }; 3],
    };
    assert_eq!(
        game.add_point(Player::Home).add_point(Player::Oponent),
        Game {
            score: Score {
                home: 15,
                oponent: 15,
            },
            stage: Stage::Normal,
            sets: [Sets {
                home: 0,
                oponent: 0,
                finished: false,
            }; 3],
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
        sets: [Sets {
            home: 0,
            oponent: 0,
            finished: false,
        }; 3],
    };
    assert_eq!(
        game.add_point(Player::Home),
        Game {
            score: Score {
                home: 0,
                oponent: 0,
            },
            stage: Stage::Deuce,
            sets: [Sets {
                home: 0,
                oponent: 0,
                finished: false,
            }; 3],
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
        sets: [Sets {
            home: 0,
            oponent: 0,
            finished: false,
        }; 3],
    };
    assert_eq!(
        game.add_point(Player::Home),
        Game {
            score: Score {
                home: 0,
                oponent: 0,
            },
            stage: Stage::Normal,
            sets: [
                Sets {
                    home: 1,
                    oponent: 0,
                    finished: false,
                },
                Sets {
                    home: 0,
                    oponent: 0,
                    finished: false,
                },
                Sets {
                    home: 0,
                    oponent: 0,
                    finished: false,
                }
            ],
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
        sets: [Sets {
            home: 0,
            oponent: 0,
            finished: false,
        }; 3],
    };
    assert_eq!(
        game.add_point(Player::Home),
        Game {
            score: Score {
                home: 0,
                oponent: 0,
            },
            stage: Stage::Normal,
            sets: [
                Sets {
                    home: 1,
                    oponent: 0,
                    finished: false,
                },
                Sets {
                    home: 0,
                    oponent: 0,
                    finished: false,
                },
                Sets {
                    home: 0,
                    oponent: 0,
                    finished: false,
                }
            ],
        }
    );
    let game2 = Game {
        score: Score {
            home: 30,
            oponent: 40,
        },
        stage: Stage::Normal,
        sets: [Sets {
            home: 0,
            oponent: 0,
            finished: false,
        }; 3],
    };
    assert_eq!(
        game2.add_point(Player::Oponent),
        Game {
            score: Score {
                home: 0,
                oponent: 0,
            },
            stage: Stage::Normal,
            sets: [
                Sets {
                    home: 0,
                    oponent: 1,
                    finished: false,
                },
                Sets {
                    home: 0,
                    oponent: 0,
                    finished: false,
                },
                Sets {
                    home: 0,
                    oponent: 0,
                    finished: false,
                }
            ],
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
        sets: [
            Sets {
                home: 5,
                oponent: 4,
                finished: false,
            },
            Sets {
                home: 0,
                oponent: 0,
                finished: false,
            },
            Sets {
                home: 0,
                oponent: 0,
                finished: false,
            },
        ],
    };
    assert_eq!(
        game.add_point(Player::Home),
        Game {
            score: Score {
                home: 0,
                oponent: 0,
            },
            stage: Stage::Normal,
            sets: [
                Sets {
                    home: 6,
                    oponent: 4,
                    finished: true,
                },
                Sets {
                    home: 0,
                    oponent: 0,
                    finished: false,
                },
                Sets {
                    home: 0,
                    oponent: 0,
                    finished: false,
                }
            ],
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
        sets: [
            Sets {
                home: 6,
                oponent: 4,
                finished: true,
            },
            Sets {
                home: 0,
                oponent: 0,
                finished: false,
            },
            Sets {
                home: 0,
                oponent: 0,
                finished: false,
            },
        ],
    };
    assert_eq!(
        game.add_point(Player::Home),
        Game {
            score: Score {
                home: 0,
                oponent: 0,
            },
            stage: Stage::Normal,
            sets: [
                Sets {
                    home: 6,
                    oponent: 4,
                    finished: true,
                },
                Sets {
                    home: 1,
                    oponent: 0,
                    finished: false,
                },
                Sets {
                    home: 0,
                    oponent: 0,
                    finished: false,
                },
            ],
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
        sets: [
            Sets {
                home: 6,
                oponent: 4,
                finished: true,
            },
            Sets {
                home: 5,
                oponent: 4,
                finished: false,
            },
            Sets {
                home: 0,
                oponent: 0,
                finished: false,
            },
        ],
    };
    assert_eq!(
        game.add_point(Player::Home),
        Game {
            score: Score {
                home: 0,
                oponent: 0,
            },
            stage: Stage::Normal,
            sets: [
                Sets {
                    home: 6,
                    oponent: 4,
                    finished: true,
                },
                Sets {
                    home: 6,
                    oponent: 4,
                    finished: true,
                },
                Sets {
                    home: 0,
                    oponent: 0,
                    finished: false,
                },
            ],
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
        sets: [
            Sets {
                home: 6,
                oponent: 5,
                finished: false,
            },
            Sets {
                home: 0,
                oponent: 0,
                finished: false,
            },
            Sets {
                home: 0,
                oponent: 0,
                finished: false,
            },
        ],
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
            sets: [
                Sets {
                    home: 6,
                    oponent: 6,
                    finished: false,
                },
                Sets {
                    home: 0,
                    oponent: 0,
                    finished: false,
                },
                Sets {
                    home: 0,
                    oponent: 0,
                    finished: false,
                },
            ],
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
        sets: [
            Sets {
                home: 6,
                oponent: 6,
                finished: false,
            },
            Sets {
                home: 0,
                oponent: 0,
                finished: false,
            },
            Sets {
                home: 0,
                oponent: 0,
                finished: false,
            },
        ],
    };
    assert_eq!(
        game.add_point(Player::Home),
        Game {
            score: Score {
                home: 0,
                oponent: 0,
            },
            stage: Stage::Normal,
            sets: [
                Sets {
                    home: 7,
                    oponent: 6,
                    finished: true,
                },
                Sets {
                    home: 0,
                    oponent: 0,
                    finished: false,
                },
                Sets {
                    home: 0,
                    oponent: 0,
                    finished: false,
                },
            ],
        }
    );
    let game2 = Game {
        score: Score {
            home: 6,
            oponent: 5,
        },
        stage: Stage::TieBreak,
        sets: [
            Sets {
                home: 6,
                oponent: 6,
                finished: false,
            },
            Sets {
                home: 0,
                oponent: 0,
                finished: false,
            },
            Sets {
                home: 0,
                oponent: 0,
                finished: false,
            },
        ],
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
            sets: [
                Sets {
                    home: 6,
                    oponent: 7,
                    finished: true,
                },
                Sets {
                    home: 0,
                    oponent: 0,
                    finished: false,
                },
                Sets {
                    home: 0,
                    oponent: 0,
                    finished: false,
                },
            ],
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
        sets: [
            Sets {
                home: 7,
                oponent: 6,
                finished: true,
            },
            Sets {
                home: 6,
                oponent: 6,
                finished: false,
            },
            Sets {
                home: 0,
                oponent: 0,
                finished: false,
            },
        ],
    };
    assert_eq!(
        game.add_point(Player::Home),
        Game {
            score: Score {
                home: 0,
                oponent: 0,
            },
            stage: Stage::Normal,
            sets: [
                Sets {
                    home: 7,
                    oponent: 6,
                    finished: true,
                },
                Sets {
                    home: 7,
                    oponent: 6,
                    finished: true,
                },
                Sets {
                    home: 0,
                    oponent: 0,
                    finished: false,
                },
            ],
        }
    );

    // OPONENT WINS
    let game2 = Game {
        score: Score {
            home: 6,
            oponent: 5,
        },
        stage: Stage::TieBreak,
        sets: [
            Sets {
                home: 6,
                oponent: 7,
                finished: true,
            },
            Sets {
                home: 6,
                oponent: 6,
                finished: false,
            },
            Sets {
                home: 0,
                oponent: 0,
                finished: false,
            },
        ],
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
            sets: [
                Sets {
                    home: 6,
                    oponent: 7,
                    finished: true,
                },
                Sets {
                    home: 6,
                    oponent: 7,
                    finished: true,
                },
                Sets {
                    home: 0,
                    oponent: 0,
                    finished: false,
                },
            ],
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
        sets: [
            Sets {
                home: 7,
                oponent: 6,
                finished: true,
            },
            Sets {
                home: 6,
                oponent: 5,
                finished: false,
            },
            Sets {
                home: 0,
                oponent: 0,
                finished: false,
            },
        ],
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
            sets: [
                Sets {
                    home: 7,
                    oponent: 6,
                    finished: true,
                },
                Sets {
                    home: 7,
                    oponent: 5,
                    finished: true,
                },
                Sets {
                    home: 0,
                    oponent: 1,
                    finished: false,
                },
            ],
        }
    );
}
// }
