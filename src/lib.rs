// TODO MAKE GAME WIN FUNCTION
// TODO MAKE TIE BREAK FUNCTION
#![feature(array_methods)]
#![feature(array_map)]
fn normal_game(players: &FullGame, who_scored: Player) -> FullGame {
    let play = match players.game {
        // check for who_scored for HOME
        [home, oponent] if home.0 == who_scored && home.1 < 30 => {
            [(home.0, home.1 + 15), (oponent)]
        }
        [home, oponent] if home.0 == who_scored && home.1 == 30 => {
            [(home.0, home.1 + 10), (oponent)]
        }
        // If it gives 41 its a game win and it will be processed later
        [home, oponent] if home.0 == who_scored && home.1 == 40 && oponent.1 <= 30 => {
            [(home.0, home.1 + 1), (oponent)]
        }

        // check for who_scored for Oponent
        [home, oponent] if oponent.0 == who_scored && oponent.1 < 30 => {
            [(home), (oponent.0, oponent.1 + 15)]
        }
        [home, oponent] if oponent.0 == who_scored && oponent.1 == 30 => {
            [(home), (oponent.0, oponent.1 + 10)]
        }
        // If it gives 41 its a game win and it will be processed later
        [home, oponent] if oponent.0 == who_scored && oponent.1 == 40 && home.1 <= 30 => {
            [(home), (oponent.0, oponent.1 + 1)]
        }
        _ => [players.game[0], players.game[1]],
    };

    return FullGame {
        game: play,
        stage: players.stage,
        set: players.set,
    };
}

fn deuce_game(mut players: FullGame, who_scored: Player) -> FullGame {
    // adds point to the correct player
    players = match who_scored {
        Player::Home => FullGame {
            game: [(Player::Home, players.game[0].1 + 1), players.game[1]],
            stage: players.stage,
            set: players.set,
        },
        Player::Oponent => FullGame {
            game: [players.game[0], (Player::Oponent, players.game[1].1 + 1)],
            stage: players.stage,
            set: players.set,
        },
    };

    // Difference in point in between PLayer Home and Player Oponent
    let difference = players.game[0].1 - players.game[1].1;
    println!("Differrence: {}", difference);
    // if the difference in beetween point is enough it goes to game add
    if difference >= 2 || difference <= -2 {
        if players.game[0].1 > players.game[1].1 {
            return FullGame {
                game: [(Player::Home, 0), (Player::Oponent, 0)],
                stage: Stage::Normal,
                set: [(1, 0, false); 3],
            };
        } else {
            return FullGame {
                game: [(Player::Home, 0), (Player::Oponent, 0)],
                stage: Stage::Normal,
                set: [(0, 1, false); 3],
            };
        }
    } else {
        return players;
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Stage {
    Normal,
    Deuce,
    TieBreak,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Player {
    Home,
    Oponent,
}

#[derive(PartialEq, Debug)]
pub struct FullGame {
    pub game: [(Player, i8); 2],
    pub stage: Stage,
    pub set: [(u8, u8, bool); 3],
}

impl FullGame {
    // creates a new game
    pub fn new() -> Self {
        FullGame {
            game: [(Player::Home, 0), (Player::Oponent, 0)],
            stage: Stage::Normal,
            set: [(0, 0, false); 3],
        }
    }

    pub fn add_point(self, point: Player) -> Self {
        let play = match self.stage {
            Stage::Normal => normal_game(&self, point),
            Stage::Deuce => deuce_game(self, point),
            _ => self,
            // Stage::TieBreak => tie_break(),
        };

        return play;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use pretty_assertions::assert_eq;
    #[test]
    fn initialize() {
        assert_eq!(
            FullGame::new(),
            FullGame {
                game: [(Player::Home, 0), (Player::Oponent, 0)],
                stage: Stage::Normal,
                set: [(0, 0, false); 3]
            }
        )
    }
    #[test]
    fn add_point_test() {
        let game = FullGame {
            game: [(Player::Home, 0), (Player::Oponent, 0)],
            stage: Stage::Normal,
            set: [(0, 0, false); 3],
        };
        assert_eq!(
            game.add_point(Player::Home).add_point(Player::Oponent),
            FullGame {
                game: [(Player::Home, 15), (Player::Oponent, 15)],
                stage: Stage::Normal,
                set: [(0, 0, false); 3],
            }
        );
    }
    #[test]
    fn check_deuce() {
        let game = FullGame {
            game: [(Player::Home, 1), (Player::Oponent, 0)],
            stage: Stage::Deuce,
            set: [(0, 0, false); 3],
        };
        assert_eq!(
            game.add_point(Player::Home),
            FullGame {
                game: [(Player::Home, 0), (Player::Oponent, 0)],
                stage: Stage::Normal,
                set: [(1, 0, false); 3],
            }
        );
    }
}
