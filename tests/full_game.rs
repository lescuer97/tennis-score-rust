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
    // }
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
    // }
}
