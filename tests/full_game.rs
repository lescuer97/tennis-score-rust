// Testing of GameScore Struct
// #[cfg(test)]
// mod tests {
//     use super::*;

// pub mod tenis_actions;
use tennis_score::tenis_actions::{FullGame, Game, Player, Score, Sets, Stage};
#[test]
fn initialize() {
    let game: FullGame = FullGame::new();
    // game.add_point(Player::Home);

    //     // assert_eq!(true, true);
    assert_eq!(
        game,
        FullGame {
            game_score: Game {
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
            },
            list_history: [].to_vec(),
        }
    )
}
#[test]
fn add_point() {
    let game: FullGame = FullGame {
        game_score: Game {
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
        },
        list_history: [].to_vec(),
    };

    assert_eq!(
        game.add_point(Player::Home),
        FullGame {
            game_score: Game {
                score: Score {
                    home: 15,
                    oponent: 0,
                },
                stage: Stage::Normal,
                sets: [Sets {
                    home: 0,
                    oponent: 0,
                    finished: false,
                }; 3]
            },
            list_history: [Game {
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
            },]
            .to_vec(),
        }
    )
}

#[test]
fn roll_back_last_point() {
    let game: FullGame = FullGame {
        game_score: Game {
            score: Score {
                home: 15,
                oponent: 0,
            },
            stage: Stage::Normal,
            sets: [Sets {
                home: 0,
                oponent: 0,
                finished: false,
            }; 3],
        },
        list_history: [Game {
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
        }]
        .to_vec(),
    };

    assert_eq!(
        game.roll_back_last_point(),
        FullGame {
            game_score: Game {
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
            },
            list_history: [].to_vec(),
        }
    )
}
#[test]
fn roll_back_to_specific_point_index_2() {
    let game: FullGame = FullGame {
        game_score: Game {
            score: Score {
                home: 5,
                oponent: 5,
            },
            stage: Stage::Normal,
            sets: [Sets {
                home: 0,
                oponent: 0,
                finished: false,
            }; 3],
        },
        list_history: [
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
                }; 3],
            },
            Game {
                score: Score {
                    home: 1,
                    oponent: 1,
                },
                stage: Stage::Normal,
                sets: [Sets {
                    home: 1,
                    oponent: 1,
                    finished: false,
                }; 3],
            },
            Game {
                score: Score {
                    home: 2,
                    oponent: 2,
                },
                stage: Stage::Normal,
                sets: [Sets {
                    home: 2,
                    oponent: 2,
                    finished: false,
                }; 3],
            },
            Game {
                score: Score {
                    home: 3,
                    oponent: 3,
                },
                stage: Stage::Normal,
                sets: [Sets {
                    home: 3,
                    oponent: 3,
                    finished: false,
                }; 3],
            },
            Game {
                score: Score {
                    home: 4,
                    oponent: 4,
                },
                stage: Stage::Normal,
                sets: [Sets {
                    home: 4,
                    oponent: 4,
                    finished: false,
                }; 3],
            },
        ]
        .to_vec(),
    };

    assert_eq!(
        game.roll_back_to_specific_point(2),
        FullGame {
            game_score: Game {
                score: Score {
                    home: 2,
                    oponent: 2,
                },
                stage: Stage::Normal,
                sets: [Sets {
                    home: 2,
                    oponent: 2,
                    finished: false,
                }; 3],
            },
            list_history: [
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
                    }; 3],
                },
                Game {
                    score: Score {
                        home: 1,
                        oponent: 1,
                    },
                    stage: Stage::Normal,
                    sets: [Sets {
                        home: 1,
                        oponent: 1,
                        finished: false,
                    }; 3],
                },
            ]
            .to_vec(),
        }
    );
}

#[test]
fn roll_back_to_specific_point_index_0() {
    let game: FullGame = FullGame {
        game_score: Game {
            score: Score {
                home: 15,
                oponent: 0,
            },
            stage: Stage::Normal,
            sets: [Sets {
                home: 0,
                oponent: 0,
                finished: false,
            }; 3],
        },
        list_history: [
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
                }; 3],
            },
            Game {
                score: Score {
                    home: 0,
                    oponent: 15,
                },
                stage: Stage::Normal,
                sets: [Sets {
                    home: 0,
                    oponent: 0,
                    finished: false,
                }; 3],
            },
            Game {
                score: Score {
                    home: 30,
                    oponent: 0,
                },
                stage: Stage::Normal,
                sets: [Sets {
                    home: 0,
                    oponent: 0,
                    finished: false,
                }; 3],
            },
            Game {
                score: Score {
                    home: 15,
                    oponent: 0,
                },
                stage: Stage::Normal,
                sets: [Sets {
                    home: 0,
                    oponent: 0,
                    finished: false,
                }; 3],
            },
        ]
        .to_vec(),
    };

    assert_eq!(
        game.roll_back_to_specific_point(0),
        FullGame {
            game_score: Game {
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
            },
            list_history: [].to_vec(),
        }
    );
}
