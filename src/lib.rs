// TODO CHECK IF THERE IS A DEUCE CHECK WHEN IT'S WON
// TODO IF THERE IS TIE BREAK CHECK IF THERE IS ADVANTAGE OF TWO POINT AND 7 OR OVER

#![feature(array_methods)]
#![feature(array_map)]

fn check_deuce(check: FullGame) -> FullGame {
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

fn game_win(check: &mut FullGame, point: u8) -> FullGame {
 // round is for counting the completed sets
 let mut round:i8 = 0;
 let mut action: i8 = 0;
 let adder = |round:i8| {  round + 1};
 // checks if the last set is done and moves to the next 
 let  point_added: [(u8,u8);3] = check.set.map(| a: (u8,u8) | // FIXME FIRST SET REGISTER FAILING
                                                             // if a.0 >= 6 || a.1 >= 6  {
                                                             //     round = adder(round);
                                                             //     a 
                                                             // }else if point == 0 {
                                                             //     // checks if other sets have been looped and actions have taken place
                                                             //     if round ==  action && action >= 0 { 
                                                             //          a 
                                                             //     } else {
                                                             //         action = adder(action);
                                                             //         (a.0 + 1, a.1)
                                                             //     }                                                                      
                                                              
                                                             // } else if point == 1 {
                                                             //     if round ==  action && action >= 0 { 
                                                             //          a 
                                                             //     } else {
                                                             //         action = adder(action);
                                                             //         (a.0,a.1 + 1)
                                                             //     }                                                                    
                                                             // }else { a }
                                                         );

    return FullGame {
        game: ((check.game.0 .0, 0), (check.game.1 .0, 0)),
        stage: Stage::Normal,
        set: check.set,
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
            (hm, op) if hm.0 == point && hm.1 < 30 => ((hm.0, hm.1 + 15), (op)),
            (hm, op) if hm.0 == point && hm.1 == 30 => ((hm.0, hm.1 + 10), (op)),
            // If it gives 41 its a game win and it will be processed later
            (hm, op) if hm.0 == point && hm.1 == 40 && op.1 <= 30 => ((hm.0, hm.1 + 1), (op)),

            // check for point for Oponent
            (hm, op) if op.0 == point && op.1 < 30 => ((hm), (op.0, op.1 + 15)),
            (hm, op) if op.0 == point && op.1 == 30 => ((hm), (op.0, op.1 + 10)),
            // If it gives 41 its a game win and it will be processed later
            (hm, op) if op.0 == point && op.1 == 40 && hm.1 <= 30 => ((hm), (op.0, op.1 + 1)),
            _ => (self.game.0, self.game.1),
        };
        // checks gamewhin for first player
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
    }
}
