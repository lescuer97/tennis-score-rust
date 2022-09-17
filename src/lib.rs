//! # tennis-score
//!
//! `tennis-score` is a little "in development" app that lets you play tennis and in the future
//! be able to be put in a micro controller so you can carry it to play games.
pub mod tenis_actions {

    /// Denotes in what stage of a set you are.
    #[derive(PartialEq, Debug, Clone, Copy)]
    pub enum Stage {
        Normal,
        Deuce,
        TieBreak,
    }

    // use tinyvec::ArrayVec;
    #[derive(PartialEq, Debug, Clone)]

    pub struct FullGame {
        pub game_score: Game,
        pub list_history: Vec<Game>,
    }
    impl FullGame {
        /// creates a new game fromscratch
        pub fn new() -> Self {
            let vector: Vec<Game> = Vec::new();

            FullGame {
                game_score: Game::new(),
                list_history: vector,
            }
        }
        /// adds point to Gamescore, uses the inside function of Gamescore
        pub fn add_point(mut self, point: Player) -> FullGame {
            // adds score to the list before updating
            self.list_history.push(self.game_score);

            // adds point
            let new_score: Game = self.game_score.add_point(point);

            FullGame {
                game_score: new_score,
                ..self
            }
        }

        /// adds point to Gamescore, uses the inside function of Gamescore
        pub fn roll_back_last_point(mut self) -> FullGame {
            if self.list_history.is_empty() {
                return self;
            } else {
                let rolled_back_point = self.list_history.pop();

                FullGame {
                    game_score: rolled_back_point.unwrap(),
                    ..self
                }
            }
        }
    }
    #[derive(PartialEq, Debug, Clone, Copy)]
    pub struct Sets {
        pub home: i8,
        pub oponent: i8,
        pub finished: bool,
    }

    impl Sets {
        pub fn new() -> Self {
            return Sets {
                home: 0,
                oponent: 0,
                finished: false,
            };
        }

        pub fn add_point(self, who_scored: Player) -> Sets {
            match who_scored {
                Player::Home => {
                    return Sets {
                        home: self.home + 1,
                        ..self
                    }
                }
                Player::Oponent => {
                    return Sets {
                        oponent: self.oponent + 1,
                        ..self
                    }
                }
            };
        }
    }
    #[derive(PartialEq, Debug, Clone, Copy)]
    /// Current score for the game
    pub struct Score {
        pub home: i8,
        pub oponent: i8,
    }
    impl Score {
        /// Creates a new game
        pub fn new() -> Self {
            return Score {
                home: 0,
                oponent: 0,
            };
        }
        /// Adds point to Game
        pub fn normal_point(self, who_scored: Player) -> Score {
            let score_to_change: i8 = match who_scored {
                Player::Home => self.home,
                Player::Oponent => self.oponent,
            };

            let new_score: i8 = match score_to_change {
                0 => 15,
                15 => 30,
                30 => 40,
                // if is a game win it will show 41
                40 => 41,
                _ => score_to_change,
            };
            match who_scored {
                Player::Home => {
                    return Score {
                        home: new_score,
                        ..self
                    }
                }
                Player::Oponent => {
                    return Score {
                        oponent: new_score,
                        ..self
                    }
                }
            };
        }
        pub fn deuce_tiebreak_point(self, who_scored: Player) -> Score {
            match who_scored {
                Player::Home => {
                    return Score {
                        home: self.home + 1,
                        ..self
                    }
                }
                Player::Oponent => {
                    return Score {
                        oponent: self.oponent + 1,
                        ..self
                    }
                }
            };
        }

        pub fn get_score_diference(self) -> i8 {
            return self.home - self.oponent;
        }
    }

    /// Structures the game for the scores and the set
    #[derive(PartialEq, Debug, Clone, Copy)]
    pub struct Game {
        // First Tupple is Home
        pub score: Score,
        pub stage: Stage,
        // first is the home player and second the oponent
        // the bool is to check if the set is finished
        pub sets: [Sets; 3],
    }
    /// Denotes the type of player
    #[derive(PartialEq, Debug, Clone, Copy)]
    pub enum Player {
        Home,
        Oponent,
    }
    impl Game {
        /// creates a new initial game.
        pub fn new() -> Self {
            Game {
                score: Score::new(),
                stage: Stage::Normal,
                sets: [Sets::new(); 3],
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
        fn normal_point(self, who_scored: Player) -> Game {
            let play = self.score.normal_point(who_scored);

            // if the game is 40 - 40 it will mark it as deuce
            if play.home == 40 && play.oponent == 40 {
                return Game {
                    score: Score::new(),
                    stage: Stage::Deuce,
                    ..self
                };
            }
            let updated_game: Game = Game {
                score: play,
                ..self
            };
            // if is a game win it will show 41 and game win will happen.
            if play.home == 41 {
                return updated_game.game_win(Player::Home);
            }

            // if is a game win it will show 41 and game win will happen.
            if play.oponent == 41 {
                return updated_game.game_win(Player::Oponent);
            }
            return Game {
                score: play,
                ..self
            };
        }

        fn deuce_point(self, who_scored: Player) -> Game {
            // adds point to the correct player
            let updated_game = Game {
                score: self.score.deuce_tiebreak_point(who_scored),
                ..self
            };

            // Difference in point in between PLayer Home and Player Oponent
            let difference = updated_game.score.get_score_diference();

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
        fn tie_break_point(self, who_scored: Player) -> Game {
            let updated_game = Game {
                score: self.score.deuce_tiebreak_point(who_scored),
                ..self
            };

            let difference = updated_game.score.get_score_diference();
            // HOME Wins Game, if gets to six and difference is bigger than 2 point
            if difference == 2 && updated_game.score.home >= 6 {
                return updated_game.game_win(Player::Home);
            }

            // Oposition wins game, if gets to six and difference is bigger than 2 point
            if difference == -2 && updated_game.score.oponent >= 6 {
                return updated_game.game_win(Player::Oponent);
            }
            return updated_game;
        }

        /// adds game point if the game was won
        fn game_win(self, who_scored: Player) -> Game {
            let mut done: bool = false;

            // check for sets that are finished and only writes on the first one that is not finished
            let mut score: [Sets; 3] = match who_scored {
                // loops inside the array of sets
                // checks that the game is not already done and the point is not added
                Player::Home => self.sets.map(|set: Sets| {
                    if !set.finished && !done {
                        done = true;
                        return Sets {
                            home: set.home + 1,
                            ..set
                        };
                    } else {
                        set
                    }
                }),
                // loops inside the array of sets
                // checks that the game is not already done and the point is not added
                Player::Oponent => self.sets.map(|set: Sets| {
                    if !set.finished && !done {
                        done = true;
                        return Sets {
                            oponent: set.oponent + 1,
                            ..set
                        };
                    } else {
                        return set;
                    }
                }),
            };

            // THIS CHECKS IF SET WAS WON
            score = score.map(|set: Sets| {
                let difference: i8 = set.home - set.oponent;
                // FOR Home SET WIN
                // this checks tiebreak win
                if difference == 1
                    && set.oponent == 6
                    && !set.finished
                    && self.stage == Stage::TieBreak
                {
                    return Sets {
                        finished: true,
                        ..set
                    };
                }
                // checks normal set win
                if difference == 2 && set.oponent >= 4 && !set.finished {
                    return Sets {
                        finished: true,
                        ..set
                    };
                }
                // FOR OPPOSITION SET WIN
                // this check tiebreak win
                if difference == -1
                    && set.home == 6
                    && !set.finished
                    && self.stage == Stage::TieBreak
                {
                    return Sets {
                        finished: true,
                        ..set
                    };
                }
                // checks normal set win
                if difference == -2 && set.home >= 4 && !set.finished {
                    return Sets {
                        finished: true,
                        ..set
                    };
                }
                return set;
            });

            let mut activate_tie_break: bool = false;

            // THIS CHECKS IF there is a tieBreak and sets the tie_break up if 6 - 6 and not already on tiebreak
            score = score.map(|set: Sets| {
                if set.home == 6 && set.oponent == 6 && self.stage != Stage::TieBreak {
                    activate_tie_break = true;
                    return set;
                } else {
                    return set;
                }
            });

            if activate_tie_break {
                return Game {
                    score: Score::new(),
                    stage: Stage::TieBreak,
                    sets: score,
                };
            }

            return Game {
                score: Score::new(),
                stage: Stage::Normal,
                sets: score,
            };
        }
    }
}
