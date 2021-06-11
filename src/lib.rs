#![feature(array_methods)]
#![feature(array_map)]

fn game_win(whole_game: FullGame, who_scored: Player) -> FullGame {
    let mut done: bool = false;

    let mut score: [(i8, i8, bool); 3] = match who_scored {
        Player::Home => whole_game.set.map(|game: (i8, i8, bool)| {
            if !game.2 && !done {
                done = true;
                return (game.0 + 1, game.1, game.2);
            } else {
                game
            }
        }),
        Player::Oponent => whole_game.set.map(|game: (i8, i8, bool)| {
            if !game.2 && !done {
                done = true;
                return (game.0, game.1 + 1, game.2);
            } else {
                return game;
            }
        }),
    };

    score = score.map(|game: (i8, i8, bool)| {
        let difference: i8 = game.0 - game.1;
        if difference == 2 && game.1 >= 4 && !game.2 {
            return (game.0, game.1, true);
        }
        if difference == -2 && game.0 >= 4 && !game.2 {
            return (game.0, game.1, true);
        }
        return game;
    });
    println!("{:?}", score);
    return FullGame {
        score: [(Player::Home, 0), (Player::Oponent, 0)],
        stage: Stage::Normal,
        set: score,
    };
}

fn tie_break_point(whole_game: FullGame, who_scored: Player) -> FullGame {
    let play = match who_scored {
        // HOME
        Player::Home => [
            (whole_game.score[0].0, whole_game.score[0].1 + 1),
            whole_game.score[1],
        ],

        // OPPONENT
        Player::Oponent => [
            whole_game.score[0],
            (whole_game.score[1].0, whole_game.score[1].1 + 1),
        ],
    };
    let updated_game = FullGame {
        score: play,
        stage: whole_game.stage,
        set: whole_game.set,
    };

    let difference = updated_game.score[0].1 - updated_game.score[1].1;
    // HOME Wins Game, if gets to six and difference is bigger than 2 point
    if difference == 2 && updated_game.score[0].1 >= 6 {
        return game_win(updated_game, Player::Home);
    }

    // Oposition wins game, if gets to six and difference is bigger than 2 point
    if difference == -2 && updated_game.score[1].1 >= 6 {
        return game_win(updated_game, Player::Oponent);
    }
    return updated_game;
}

fn normal_point(whole_game: &FullGame, who_scored: Player) -> FullGame {
    let play = match who_scored {
        // HOME
        Player::Home if whole_game.score[0].1 == 0 => [
            (whole_game.score[0].0, whole_game.score[0].1 + 15),
            whole_game.score[1],
        ],
        Player::Home if whole_game.score[0].1 == 15 => [
            (whole_game.score[0].0, whole_game.score[0].1 + 15),
            whole_game.score[1],
        ],
        Player::Home if whole_game.score[0].1 == 30 => [
            (whole_game.score[0].0, whole_game.score[0].1 + 10),
            whole_game.score[1],
        ],
        Player::Home if whole_game.score[0].1 == 40 => [
            (whole_game.score[0].0, whole_game.score[0].1 + 1),
            whole_game.score[1],
        ],
        // OPPONENT
        Player::Oponent if whole_game.score[1].1 == 0 => [
            whole_game.score[0],
            (whole_game.score[1].0, whole_game.score[1].1 + 15),
        ],
        Player::Oponent if whole_game.score[1].1 == 15 => [
            whole_game.score[0],
            (whole_game.score[1].0, whole_game.score[1].1 + 15),
        ],
        Player::Oponent if whole_game.score[1].1 == 30 => [
            whole_game.score[0],
            (whole_game.score[1].0, whole_game.score[1].1 + 10),
        ],
        Player::Oponent if whole_game.score[1].1 == 40 => [
            whole_game.score[0],
            (whole_game.score[1].0, whole_game.score[1].1 + 1),
        ],
        _ => [whole_game.score[0], whole_game.score[1]],
    };

    if play[0].1 == 40 && play[1].1 == 40 {
        return FullGame {
            score: [(Player::Home, 0), (Player::Oponent, 0)],
            stage: Stage::Deuce,
            set: whole_game.set,
        };
    }

    if play[0].1 == 41 {
        return game_win(
            FullGame {
                score: play,
                stage: whole_game.stage,
                set: whole_game.set,
            },
            Player::Home,
        );
    }
    if play[1].1 == 41 {
        return game_win(
            FullGame {
                score: play,
                stage: whole_game.stage,
                set: whole_game.set,
            },
            Player::Oponent,
        );
    }

    return FullGame {
        score: play,
        stage: whole_game.stage,
        set: whole_game.set,
    };
}

fn deuce_point(mut whole_game: FullGame, who_scored: Player) -> FullGame {
    // adds point to the correct player
    whole_game = match who_scored {
        Player::Home => FullGame {
            score: [
                (Player::Home, whole_game.score[0].1 + 1),
                whole_game.score[1],
            ],
            stage: whole_game.stage,
            set: whole_game.set,
        },
        Player::Oponent => FullGame {
            score: [
                whole_game.score[0],
                (Player::Oponent, whole_game.score[1].1 + 1),
            ],
            stage: whole_game.stage,
            set: whole_game.set,
        },
    };

    // Difference in point in between PLayer Home and Player Oponent
    let difference = whole_game.score[0].1 - whole_game.score[1].1;

    // if the difference in beetween point is 2 it goes to game add
    if difference >= 2 || difference <= -2 {
        if whole_game.score[0].1 > whole_game.score[1].1 {
            return FullGame {
                score: [(Player::Home, 0), (Player::Oponent, 0)],
                stage: Stage::Normal,
                set: [(1, 0, false); 3],
            };
        } else {
            return FullGame {
                score: [(Player::Home, 0), (Player::Oponent, 0)],
                stage: Stage::Normal,
                set: [(0, 1, false); 3],
            };
        }
    } else {
        return whole_game;
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
    // First Tupple is Home
    pub score: [(Player, i8); 2],
    pub stage: Stage,
    // first is the home player and second the oponent
    // the bool is to check if the set is finished
    pub set: [(i8, i8, bool); 3],
}

impl FullGame {
    // creates a new game
    pub fn new() -> Self {
        FullGame {
            score: [(Player::Home, 0), (Player::Oponent, 0)],
            stage: Stage::Normal,
            set: [(0, 0, false); 3],
        }
    }
    // checks the stage of the game
    pub fn add_point(self, point: Player) -> Self {
        let play = match self.stage {
            Stage::Normal => normal_point(&self, point),
            Stage::Deuce => deuce_point(self, point),
            Stage::TieBreak => tie_break_point(self, point),
        };

        return play;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn initialize() {
        assert_eq!(
            FullGame::new(),
            FullGame {
                score: [(Player::Home, 0), (Player::Oponent, 0)],
                stage: Stage::Normal,
                set: [(0, 0, false); 3]
            }
        )
    }
    #[test]
    fn add_point_test() {
        let game = FullGame {
            score: [(Player::Home, 0), (Player::Oponent, 0)],
            stage: Stage::Normal,
            set: [(0, 0, false); 3],
        };
        assert_eq!(
            game.add_point(Player::Home).add_point(Player::Oponent),
            FullGame {
                score: [(Player::Home, 15), (Player::Oponent, 15)],
                stage: Stage::Normal,
                set: [(0, 0, false); 3],
            }
        );
    }
    #[test]
    fn make_deuce() {
        let game = FullGame {
            score: [(Player::Home, 30), (Player::Oponent, 40)],
            stage: Stage::Normal,
            set: [(0, 0, false); 3],
        };
        assert_eq!(
            game.add_point(Player::Home),
            FullGame {
                score: [(Player::Home, 0), (Player::Oponent, 0)],
                stage: Stage::Deuce,
                set: [(0, 0, false); 3],
            }
        );
    }
    #[test]
    fn solve_deuce() {
        let game = FullGame {
            score: [(Player::Home, 1), (Player::Oponent, 0)],
            stage: Stage::Deuce,
            set: [(0, 0, false); 3],
        };
        assert_eq!(
            game.add_point(Player::Home),
            FullGame {
                score: [(Player::Home, 0), (Player::Oponent, 0)],
                stage: Stage::Normal,
                set: [(1, 0, false); 3],
            }
        );
    }
    #[test]
    fn first_game_win() {
        let game = FullGame {
            score: [(Player::Home, 40), (Player::Oponent, 30)],
            stage: Stage::Normal,
            set: [(0, 0, false); 3],
        };
        assert_eq!(
            game.add_point(Player::Home),
            FullGame {
                score: [(Player::Home, 0), (Player::Oponent, 0)],
                stage: Stage::Normal,
                set: [(1, 0, false), (0, 0, false), (0, 0, false)],
            }
        );
        let game2 = FullGame {
            score: [(Player::Home, 30), (Player::Oponent, 40)],
            stage: Stage::Normal,
            set: [(0, 0, false); 3],
        };
        assert_eq!(
            game2.add_point(Player::Oponent),
            FullGame {
                score: [(Player::Home, 0), (Player::Oponent, 0)],
                stage: Stage::Normal,
                set: [(0, 1, false), (0, 0, false), (0, 0, false)],
            }
        );
    }
    #[test]
    fn check_first_game_turns_true() {
        let game = FullGame {
            score: [(Player::Home, 40), (Player::Oponent, 30)],
            stage: Stage::Normal,
            set: [(5, 4, false), (0, 0, false), (0, 0, false)],
        };
        assert_eq!(
            game.add_point(Player::Home),
            FullGame {
                score: [(Player::Home, 0), (Player::Oponent, 0)],
                stage: Stage::Normal,
                set: [(6, 4, true), (0, 0, false), (0, 0, false)],
            }
        );
    }

    #[test]
    fn second_game_win() {
        let game = FullGame {
            score: [(Player::Home, 40), (Player::Oponent, 30)],
            stage: Stage::Normal,
            set: [(6, 4, true), (0, 0, false), (0, 0, false)],
        };
        assert_eq!(
            game.add_point(Player::Home),
            FullGame {
                score: [(Player::Home, 0), (Player::Oponent, 0)],
                stage: Stage::Normal,
                set: [(6, 4, true), (1, 0, false), (0, 0, false)],
            }
        );
    }

    #[test]
    fn check_second_game_win_true() {
        let game = FullGame {
            score: [(Player::Home, 40), (Player::Oponent, 30)],
            stage: Stage::Normal,
            set: [(6, 4, true), (5, 4, false), (0, 0, false)],
        };
        assert_eq!(
            game.add_point(Player::Home),
            FullGame {
                score: [(Player::Home, 0), (Player::Oponent, 0)],
                stage: Stage::Normal,
                set: [(6, 4, true), (6, 4, true), (0, 0, false)],
            }
        );
    }
}
