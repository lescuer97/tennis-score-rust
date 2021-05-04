// TODO CHECK IF THERE IS A DEUCE CHECK WHEN IT'S WON
// TODO IF THERE IS TIE BREAK CHECK IF THERE IS ADVANTAGE OF TWO POINT AND 7 OR OVER

#![feature(array_methods)]
#![feature(array_map)]

fn check_deuce(check: FullGame) -> FullGame {
    // checks that both player have 40 points and set to Deuce
    let checked = if check.game.0 .1 == 40 && check.game.1 .1 == 40 {
        FullGame {
            game: ((check.game.0 .0, 0), (check.game.1 .0, 0)),
            stage: Stage::Deuce,
            set: check.set,
        }
    } else {
        FullGame {
            game: check.game,
            stage: check.stage,
            set: check.set,
        }
    };
    return checked;
}

// adds the game win in the correct place
fn game_win(check: &mut FullGame, point: u8) -> FullGame {
    // round is for counting the completed sets
    let mut passed_set: i8 = 0;

    // checks for action of adding point
    let mut action_taken: i8 = 0;

    // adds to the counter
    let adder = |a: i8| a + 1;

    // checks if the last set is done and moves to the next
    let point_added: [(u8, u8); 3] = check.set.map(|a: (u8, u8)| {
        // TODO REFATOR INTO OWN FUNCTION
        // checks if a set has already been won
        if a.0 >= 6 || a.1 >= 6 {
            passed_set = adder(passed_set);
            a
        // checks if the action of adding a point its been executed and return without doing anything
        } else if passed_set == action_taken && action_taken != 0 {
            a
            // checks if no action has been taken and the other cases has already been covered
        } else if action_taken == 0 {
            let result = match point {
                0 => (a.0 + 1, a.1),
                1 => (a.0, a.1 + 1),
                _ => a,
            };
            action_taken = adder(action_taken);
            result
        } else {
            a
        }
    });

    return FullGame {
        game: ((check.game.0 .0, 0), (check.game.1 .0, 0)),
        stage: Stage::Normal,
        set: point_added,
    };
}

#[derive(PartialEq, Debug)]
pub struct FullGame {
    pub game: ((Player, u8), (Player, u8)),
    pub stage: Stage,
    pub set: [(u8, u8); 3],
}

impl FullGame {
    // creates a new game
    pub fn new() -> Self {
        FullGame {
            game: ((Player::Home, 0), (Player::Oponent, 0)),
            stage: Stage::Normal,
            set: [(0, 0); 3],
        }
    }

    pub fn add_point(&mut self, point: Player) -> Self {
        // this is for matching normal game points
        let play = match self.game {
            // check for point for HOME
            (home, oponent) if home.0 == point && home.1 < 30 => ((home.0, home.1 + 15), (oponent)),
            (home, oponent) if home.0 == point && home.1 == 30 => {
                ((home.0, home.1 + 10), (oponent))
            }
            // If it gives 41 its a game win and it will be processed later
            (home, oponent) if home.0 == point && home.1 == 40 && oponent.1 <= 30 => {
                ((home.0, home.1 + 1), (oponent))
            }

            // check for point for Oponent
            (home, oponent) if oponent.0 == point && oponent.1 < 30 => {
                ((home), (oponent.0, oponent.1 + 15))
            }
            (home, oponent) if oponent.0 == point && oponent.1 == 30 => {
                ((home), (oponent.0, oponent.1 + 10))
            }
            // If it gives 41 its a game win and it will be processed later
            (home, oponent) if oponent.0 == point && oponent.1 == 40 && home.1 <= 30 => {
                ((home), (oponent.0, oponent.1 + 1))
            }
            _ => (self.game.0, self.game.1),
        };
        // checks game win for first player
        let checker = if play.0 .1 == 41 && self.stage == Stage::Normal {
            game_win(self, 0)
            // checks gamewin for second player
        } else if play.1 .1 == 41 {
            game_win(self, 1)
        } else {
            FullGame {
                game: play,
                stage: self.stage,
                set: self.set,
            }
        };
        let deuce = check_deuce(checker);
        return deuce;
        // let mut game = check_deuce(self);
        // game = check_win(&mut game);
        // return game;
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn initialize() {
        assert_eq!(
            FullGame::new(),
            FullGame {
                game: ((Player::Home, 0), (Player::Oponent, 0)),
                stage: Stage::Normal,
                set: [(0, 0); 3]
            }
        )
    }
    #[test]
    fn add_point_test() {
        let mut game = FullGame {
            game: ((Player::Home, 30), (Player::Oponent, 30)),
            stage: Stage::Normal,
            set: [(0, 0); 3],
        };

        assert_eq!(
            game.add_point(Player::Oponent),
            FullGame {
                game: ((Player::Home, 30), (Player::Oponent, 40)),
                stage: Stage::Normal,
                set: [(0, 0); 3]
            }
        );
        assert_eq!(
            game.add_point(Player::Home),
            FullGame {
                game: ((Player::Home, 40), (Player::Oponent, 30)),
                stage: Stage::Normal,
                set: [(0, 0); 3]
            }
        );
        let mut game2 = FullGame {
            game: ((Player::Home, 0), (Player::Oponent, 0)),
            stage: Stage::Normal,
            set: [(0, 0); 3],
        };
        assert_eq!(
            game2.add_point(Player::Home),
            FullGame {
                game: ((Player::Home, 15), (Player::Oponent, 0)),
                stage: Stage::Normal,
                set: [(0, 0); 3]
            }
        );
        assert_eq!(
            game2.add_point(Player::Oponent),
            FullGame {
                game: ((Player::Home, 0), (Player::Oponent, 15)),
                stage: Stage::Normal,
                set: [(0, 0); 3]
            }
        );
    }

    #[test]
    fn check_deuce() {
        let mut game = FullGame {
            game: ((Player::Home, 30), (Player::Oponent, 40)),
            stage: Stage::Normal,
            set: [(0, 0); 3],
        };

        assert_eq!(
            game.add_point(Player::Home),
            FullGame {
                game: ((Player::Home, 0), (Player::Oponent, 0)),
                stage: Stage::Deuce,
                set: [(0, 0); 3]
            }
        );
        let mut game2 = FullGame {
            game: ((Player::Home, 40), (Player::Oponent, 30)),
            stage: Stage::Normal,
            set: [(0, 0); 3],
        };

        assert_eq!(
            game2.add_point(Player::Oponent),
            FullGame {
                game: ((Player::Home, 0), (Player::Oponent, 0)),
                stage: Stage::Deuce,
                set: [(0, 0); 3]
            }
        );
    }
    #[test]
    fn check_game_win() {
        let mut game = FullGame {
            game: ((Player::Home, 30), (Player::Oponent, 40)),
            stage: Stage::Normal,
            set: [(0, 0); 3],
        };

        assert_eq!(
            game.add_point(Player::Oponent),
            FullGame {
                game: ((Player::Home, 0), (Player::Oponent, 0)),
                stage: Stage::Normal,
                set: [(0, 1), (0, 0), (0, 0)]
            }
        );
    }

    #[test]
    fn check_second_set_win() {
        let mut game = FullGame {
            game: ((Player::Home, 30), (Player::Oponent, 40)),
            stage: Stage::Normal,
            set: [(7, 5), (0, 0), (0, 0)],
        };

        assert_eq!(
            game.add_point(Player::Oponent),
            FullGame {
                game: ((Player::Home, 0), (Player::Oponent, 0)),
                stage: Stage::Normal,
                set: [(7, 5), (0, 1), (0, 0)]
            }
        );
        let mut game2 = FullGame {
            game: ((Player::Home, 40), (Player::Oponent, 30)),
            stage: Stage::Normal,
            set: [(7, 5), (0, 0), (0, 0)],
        };

        assert_eq!(
            game2.add_point(Player::Home),
            FullGame {
                game: ((Player::Home, 0), (Player::Oponent, 0)),
                stage: Stage::Normal,
                set: [(7, 5), (1, 0), (0, 0)]
            }
        );
    }
}
