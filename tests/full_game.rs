// Testing of GameScore Struct
// #[cfg(test)]
// mod tests {
//     use super::*;

// pub mod tenis_actions;
use tennis_score::tenis_actions::{FullGame, GameScore, Player, Stage};
#[test]
fn initialize() {
    let game: FullGame = FullGame::new();
    // game.add_point(Player::Home);

    //     // assert_eq!(true, true);
    assert_eq!(
        game,
        FullGame {
            game_score: GameScore {
                score: [(Player::Home, 0), (Player::Oponent, 0)],
                stage: Stage::Normal,
                sets: [(0, 0, false); 3]
            },
            list_history: [GameScore {
                score: [(Player::Home, 0), (Player::Oponent, 0)],
                stage: Stage::Normal,
                sets: [(0, 0, false); 3],
            }]
            .to_vec(),
        }
    )
    // }
}
#[test]
fn add_point() {
    let game: FullGame = FullGame {
        game_score: GameScore {
            score: [(Player::Home, 0), (Player::Oponent, 0)],
            stage: Stage::Normal,
            sets: [(0, 0, false); 3],
        },
        list_history: [GameScore {
            score: [(Player::Home, 0), (Player::Oponent, 0)],
            stage: Stage::Normal,
            sets: [(0, 0, false); 3],
        }]
        .to_vec(),
    };

    assert_eq!(
        game.add_point(Player::Home),
        FullGame {
            game_score: GameScore {
                score: [(Player::Home, 15), (Player::Oponent, 0)],
                stage: Stage::Normal,
                sets: [(0, 0, false); 3]
            },
            list_history: [
                GameScore {
                    score: [(Player::Home, 0), (Player::Oponent, 0)],
                    stage: Stage::Normal,
                    sets: [(0, 0, false); 3],
                },
                GameScore {
                    score: [(Player::Home, 15), (Player::Oponent, 0)],
                    stage: Stage::Normal,
                    sets: [(0, 0, false); 3],
                }
            ]
            .to_vec(),
        }
    )
    // }
}
