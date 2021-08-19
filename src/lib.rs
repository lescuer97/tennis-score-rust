//! # tennis-score
//!
//! `tennis-score` is a little "in development" app that lets you play tennis and in the future
//! be able to be put in a micro controller so you can carry it to play games.

/// Denotes in what stage of a set you are.
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Stage {
    Normal,
    Deuce,
    TieBreak,
}

/// Denotes the type of player
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Player {
    Home,
    Oponent,
}

/// Structures the game for the scores and the set
#[derive(PartialEq, Debug)]
pub struct FullGame {
    // First Tupple is Home
    pub score: [(Player, i8); 2],
    pub stage: Stage,
    // first is the home player and second the oponent
    // the bool is to check if the set is finished
    pub sets: [(i8, i8, bool); 3],
}

impl FullGame {
    /// creates a new initial game.
    pub fn new() -> Self {
        FullGame {
            score: [(Player::Home, 0), (Player::Oponent, 0)],
            stage: Stage::Normal,
            sets: [(0, 0, false); 3],
        }
    }
    /// Ads point to a player, adds depending on the stage of the game.
    pub fn add_point(self, point: Player) -> Self {
        let play = match self.stage {
            Stage::Normal => self.normal_point(point),
            Stage::Deuce => self.deuce_point(point),
            Stage::TieBreak => self.tie_break_point(point),
        };

        return play;
    }
    /// Adds point a Normal Stage Game, depending on the score.
    fn normal_point(self, who_scored: Player) -> FullGame {
        let play = match who_scored {
            // HOME SCORE POINTS
            Player::Home if self.score[0].1 == 0 => {
                [(self.score[0].0, self.score[0].1 + 15), self.score[1]]
            }
            Player::Home if self.score[0].1 == 15 => {
                [(self.score[0].0, self.score[0].1 + 15), self.score[1]]
            }
            Player::Home if self.score[0].1 == 30 => {
                [(self.score[0].0, self.score[0].1 + 10), self.score[1]]
            }
            // if is a game win it will show 41
            Player::Home if self.score[0].1 == 40 => {
                [(self.score[0].0, self.score[0].1 + 1), self.score[1]]
            }
            // OPPONENT SCORE POINTS
            Player::Oponent if self.score[1].1 == 0 => {
                [self.score[0], (self.score[1].0, self.score[1].1 + 15)]
            }
            Player::Oponent if self.score[1].1 == 15 => {
                [self.score[0], (self.score[1].0, self.score[1].1 + 15)]
            }
            Player::Oponent if self.score[1].1 == 30 => {
                [self.score[0], (self.score[1].0, self.score[1].1 + 10)]
            }
            Player::Oponent if self.score[1].1 == 40 => {
                [self.score[0], (self.score[1].0, self.score[1].1 + 1)]
            }
            _ => [self.score[0], self.score[1]],
        };
        // if the game is 40 - 40 it will mark it as deuce
        if play[0].1 == 40 && play[1].1 == 40 {
            return FullGame {
                score: [(Player::Home, 0), (Player::Oponent, 0)],
                stage: Stage::Deuce,
                sets: self.sets,
            };
        }
        let updated_game = FullGame {
            score: play,
            stage: self.stage,
            sets: self.sets,
        };
        // if is a game win it will show 41 and game win will happen.
        if play[0].1 == 41 {
            return updated_game.game_win(Player::Home);
        }

        // if is a game win it will show 41 and game win will happen.
        if play[1].1 == 41 {
            return updated_game.game_win(Player::Oponent);
        }
        return FullGame {
            score: play,
            stage: self.stage,
            sets: self.sets,
        };
    }
    fn deuce_point(self, who_scored: Player) -> FullGame {
        // adds point to the correct player
        let updated_game = match who_scored {
            Player::Home => FullGame {
                score: [(Player::Home, self.score[0].1 + 1), self.score[1]],
                stage: self.stage,
                sets: self.sets,
            },
            Player::Oponent => FullGame {
                score: [self.score[0], (Player::Oponent, self.score[1].1 + 1)],
                stage: self.stage,
                sets: self.sets,
            },
        };

        // Difference in point in between PLayer Home and Player Oponent
        let difference = updated_game.score[0].1 - updated_game.score[1].1;

        // if the difference in beetween point is 2 it goes to game add
        if difference >= 2 {
            return updated_game.game_win(Player::Home);
        }
        if difference <= -2 {
            return updated_game.game_win(Player::Oponent);
        }

        return updated_game;
    }
    /// Adds point in the tie break
    fn tie_break_point(self, who_scored: Player) -> FullGame {
        let play = match who_scored {
            // HOME
            Player::Home => [(self.score[0].0, self.score[0].1 + 1), self.score[1]],

            // OPPONENT
            Player::Oponent => [self.score[0], (self.score[1].0, self.score[1].1 + 1)],
        };
        let updated_game = FullGame {
            score: play,
            stage: self.stage,
            sets: self.sets,
        };

        let difference = updated_game.score[0].1 - updated_game.score[1].1;
        // HOME Wins Game, if gets to six and difference is bigger than 2 point
        if difference == 2 && updated_game.score[0].1 >= 6 {
            return updated_game.game_win(Player::Home);
        }

        // Oposition wins game, if gets to six and difference is bigger than 2 point
        if difference == -2 && updated_game.score[1].1 >= 6 {
            return updated_game.game_win(Player::Oponent);
        }
        return updated_game;
    }

    /// adds game point if the game was won
    fn game_win(self, who_scored: Player) -> FullGame {
        let mut done: bool = false;

        // check for sets that are finished and only writes on the first one that is not finished
        let mut score: [(i8, i8, bool); 3] = match who_scored {
            // loops inside the array of sets
            // checks that the game is not already done and the point is not added
            Player::Home => self.sets.map(|set: (i8, i8, bool)| {
                if !set.2 && !done {
                    done = true;
                    return (set.0 + 1, set.1, set.2);
                } else {
                    set
                }
            }),
            // loops inside the array of sets
            // checks that the game is not already done and the point is not added
            Player::Oponent => self.sets.map(|set: (i8, i8, bool)| {
                if !set.2 && !done {
                    done = true;
                    return (set.0, set.1 + 1, set.2);
                } else {
                    return set;
                }
            }),
        };

        // THIS CHECKS IF SET WAS WON
        score = score.map(|set: (i8, i8, bool)| {
            let difference: i8 = set.0 - set.1;
            // FOR Home SET WIN
            // this checks tiebreak win
            if difference == 1 && set.1 == 6 && !set.2 && self.stage == Stage::TieBreak {
                return (set.0, set.1, true);
            }
            // checks normal set win
            if difference == 2 && set.1 >= 4 && !set.2 {
                return (set.0, set.1, true);
            }
            // FOR OPPOSITION SET WIN
            // this check tiebreak win
            if difference == -1 && set.0 == 6 && !set.2 && self.stage == Stage::TieBreak {
                return (set.0, set.1, true);
            }
            // checks normal set win
            if difference == -2 && set.0 >= 4 && !set.2 {
                return (set.0, set.1, true);
            }
            return set;
        });

        let mut activate_tie_break: bool = false;

        // THIS CHECKS IF there is a tieBreak and sets the tie_break up if 6 - 6 and not already on tiebreak
        score = score.map(|set: (i8, i8, bool)| {
            if set.0 == 6 && set.1 == 6 && self.stage != Stage::TieBreak {
                activate_tie_break = true;
                return set;
            } else {
                return set;
            }
        });

        if activate_tie_break {
            return FullGame {
                score: [(Player::Home, 0), (Player::Oponent, 0)],
                stage: Stage::TieBreak,
                sets: score,
            };
        }

        return FullGame {
            score: [(Player::Home, 0), (Player::Oponent, 0)],
            stage: Stage::Normal,
            sets: score,
        };
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
                sets: [(0, 0, false); 3]
            }
        )
    }
    #[test]
    fn add_point_test() {
        let game = FullGame {
            score: [(Player::Home, 0), (Player::Oponent, 0)],
            stage: Stage::Normal,
            sets: [(0, 0, false); 3],
        };
        assert_eq!(
            game.add_point(Player::Home).add_point(Player::Oponent),
            FullGame {
                score: [(Player::Home, 15), (Player::Oponent, 15)],
                stage: Stage::Normal,
                sets: [(0, 0, false); 3],
            }
        );
    }
    #[test]
    fn make_deuce() {
        let game = FullGame {
            score: [(Player::Home, 30), (Player::Oponent, 40)],
            stage: Stage::Normal,
            sets: [(0, 0, false); 3],
        };
        assert_eq!(
            game.add_point(Player::Home),
            FullGame {
                score: [(Player::Home, 0), (Player::Oponent, 0)],
                stage: Stage::Deuce,
                sets: [(0, 0, false); 3],
            }
        );
    }
    #[test]
    fn solve_deuce() {
        let game = FullGame {
            score: [(Player::Home, 1), (Player::Oponent, 0)],
            stage: Stage::Deuce,
            sets: [(0, 0, false); 3],
        };
        assert_eq!(
            game.add_point(Player::Home),
            FullGame {
                score: [(Player::Home, 0), (Player::Oponent, 0)],
                stage: Stage::Normal,
                sets: [(1, 0, false), (0, 0, false), (0, 0, false)],
            }
        );
    }
    #[test]
    fn first_game_win() {
        let game = FullGame {
            score: [(Player::Home, 40), (Player::Oponent, 30)],
            stage: Stage::Normal,
            sets: [(0, 0, false); 3],
        };
        assert_eq!(
            game.add_point(Player::Home),
            FullGame {
                score: [(Player::Home, 0), (Player::Oponent, 0)],
                stage: Stage::Normal,
                sets: [(1, 0, false), (0, 0, false), (0, 0, false)],
            }
        );
        let game2 = FullGame {
            score: [(Player::Home, 30), (Player::Oponent, 40)],
            stage: Stage::Normal,
            sets: [(0, 0, false); 3],
        };
        assert_eq!(
            game2.add_point(Player::Oponent),
            FullGame {
                score: [(Player::Home, 0), (Player::Oponent, 0)],
                stage: Stage::Normal,
                sets: [(0, 1, false), (0, 0, false), (0, 0, false)],
            }
        );
    }
    #[test]
    fn check_first_game_turns_true() {
        let game = FullGame {
            score: [(Player::Home, 40), (Player::Oponent, 30)],
            stage: Stage::Normal,
            sets: [(5, 4, false), (0, 0, false), (0, 0, false)],
        };
        assert_eq!(
            game.add_point(Player::Home),
            FullGame {
                score: [(Player::Home, 0), (Player::Oponent, 0)],
                stage: Stage::Normal,
                sets: [(6, 4, true), (0, 0, false), (0, 0, false)],
            }
        );
    }

    #[test]
    fn second_game_win() {
        let game = FullGame {
            score: [(Player::Home, 40), (Player::Oponent, 30)],
            stage: Stage::Normal,
            sets: [(6, 4, true), (0, 0, false), (0, 0, false)],
        };
        assert_eq!(
            game.add_point(Player::Home),
            FullGame {
                score: [(Player::Home, 0), (Player::Oponent, 0)],
                stage: Stage::Normal,
                sets: [(6, 4, true), (1, 0, false), (0, 0, false)],
            }
        );
    }

    #[test]
    fn check_second_game_win_true() {
        let game = FullGame {
            score: [(Player::Home, 40), (Player::Oponent, 30)],
            stage: Stage::Normal,
            sets: [(6, 4, true), (5, 4, false), (0, 0, false)],
        };
        assert_eq!(
            game.add_point(Player::Home),
            FullGame {
                score: [(Player::Home, 0), (Player::Oponent, 0)],
                stage: Stage::Normal,
                sets: [(6, 4, true), (6, 4, true), (0, 0, false)],
            }
        );
    }
    #[test]
    fn check_deuce_and_then_tie_break() {
        let game = FullGame {
            score: [(Player::Home, 30), (Player::Oponent, 30)],
            stage: Stage::Normal,
            sets: [(6, 5, false), (0, 0, false), (0, 0, false)],
        };
        assert_eq!(
            game.add_point(Player::Home)
                .add_point(Player::Oponent)
                .add_point(Player::Oponent)
                .add_point(Player::Oponent),
            FullGame {
                score: [(Player::Home, 0), (Player::Oponent, 0)],
                stage: Stage::TieBreak,
                sets: [(6, 6, false), (0, 0, false), (0, 0, false)],
            }
        );
    }
    #[test]
    fn solve_tie_break() {
        let game = FullGame {
            score: [(Player::Home, 6), (Player::Oponent, 5)],
            stage: Stage::TieBreak,
            sets: [(6, 6, false), (0, 0, false), (0, 0, false)],
        };
        assert_eq!(
            game.add_point(Player::Home),
            FullGame {
                score: [(Player::Home, 0), (Player::Oponent, 0)],
                stage: Stage::Normal,
                sets: [(7, 6, true), (0, 0, false), (0, 0, false)],
            }
        );
        let game2 = FullGame {
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
            FullGame {
                score: [(Player::Home, 0), (Player::Oponent, 15)],
                stage: Stage::Normal,
                sets: [(6, 7, true), (0, 0, false), (0, 0, false)],
            }
        );
    }

    #[test]
    fn long_tie_break_in_second_set() {
        // HOME WINS
        let game = FullGame {
            score: [(Player::Home, 6), (Player::Oponent, 5)],
            stage: Stage::TieBreak,
            sets: [(7, 6, true), (6, 6, false), (0, 0, false)],
        };
        assert_eq!(
            game.add_point(Player::Home),
            FullGame {
                score: [(Player::Home, 0), (Player::Oponent, 0)],
                stage: Stage::Normal,
                sets: [(7, 6, true), (7, 6, true), (0, 0, false)],
            }
        );

        // OPONENT WINS
        let game2 = FullGame {
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
            FullGame {
                score: [(Player::Home, 0), (Player::Oponent, 15)],
                stage: Stage::Normal,
                sets: [(6, 7, true), (6, 7, true), (0, 0, false)],
            }
        );
    }

    #[test]
    fn third_set_starts_and_oponent_wins_first_deuce() {
        let game = FullGame {
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
            FullGame {
                score: [(Player::Home, 0), (Player::Oponent, 0)],
                stage: Stage::Normal,
                sets: [(7, 6, true), (7, 5, true), (0, 1, false)],
            }
        );
    }
}
