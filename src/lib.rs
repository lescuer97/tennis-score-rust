

    fn check_deuce(check: FullGame) -> FullGame {
     let  checked=   if check.game.0.1 == 40 && check.game.1.1 == 40 {
                 FullGame {
                    game: ((check.game.0.0, 0),(check.game.1.0, 0)),
                    stage: Stage::Deuce,
                    set: check.set
                }
            } else {
                 FullGame {
                    game: check.game,
                    stage: check.stage,
                    set: check.set
            } };
            return checked;
    }

//  TODO ADD GAMES FOR SET
#[derive(PartialEq, Debug)]
pub struct FullGame  {
    pub game: ((Player,u8) ,(Player, u8)),
    pub stage: Stage,
    pub set: [(u8,u8);3],
}

impl  FullGame {
    // creates a new game
  pub  fn new() -> Self  {
        FullGame {
            game: ((Player::Home, 0) ,(Player::Oponent, 0)),
            stage: Stage::Normal,
            set: [(0,0);3]
        }
    }

    pub fn add_point(&mut self, point: Player ) -> Self {
        // this is for matching normal game points
    let play = match self.game {
        // check for point for HOME
        (x,op ) if x.0 == point && x.1 < 30 => ((x.0, x.1 + 15),(op)),
        (x,op ) if x.0 == point && x.1 == 30 => ((x.0, x.1 + 10),(op)),
        // If it gives 41 its a game win and it will be processed later
        (x,op ) if x.0 == point && x.1 == 40 && op.1 <=30 => ((x.0, x.1 + 1),(op)),
       
        // check for point for Oponent
        (x,op ) if op.0 == point && op.1 < 30 => ((x),(op.0, op.1 + 15)),
        (x,op ) if op.0 == point && op.1 == 30 => ((x),(op.0, op.1 + 10)),
         // If it gives 41 its a game win and it will be processed later
        (x,op ) if op.0 == point && op.1 == 40 && x.1 <= 30 => ((x),(op.0, op.1 + 1)),
        _ => (self.game.0, self.game.1),
    };

 let checker = if play.0.1 == 41 && self.stage == Stage::Normal {
                         game_win(self, 0)
                    } else if play.1.1 == 41 {
                         game_win(self, 1)
                    } else {
                         FullGame {
                            game: play,
                            stage: self.stage,
                            set: self.set
                        }
                    };
    let deuce = check_deuce(checker);
    return deuce
    // let mut game = check_deuce(self);
    // game = check_win(&mut game);
    // return game;
    }
}

#[derive(PartialEq, Debug,Clone, Copy)]
pub enum Stage {
    Normal,
    Deuce,
    TieBreak,
}

#[derive(PartialEq, Debug,Clone, Copy)]
pub enum Player {
    Home,
    Oponent,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn initialize(){
     
        assert_eq!(FullGame::new(), FullGame {
            game: ((Player::Home, 0) ,(Player::Oponent, 0)),
            stage: Stage::Normal,
            set: [(0,0);3]
        })
    }
    #[test]
    fn add_point_test() {
        let mut game = FullGame  {
            game: ((Player::Home, 30) ,(Player::Oponent, 30)),
            stage: Stage::Normal,
            set: [(0,0);3]
        };

        assert_eq!( game.add_point(Player::Oponent), FullGame  {
            game: ((Player::Home, 30) ,(Player::Oponent, 40)),
            stage: Stage::Normal,
            set: [(0,0);3]
        });
    }

    #[test]
    fn check_deuce() {
        let mut game = FullGame  {
            game: ((Player::Home, 30) ,(Player::Oponent, 40)),
            stage: Stage::Normal,
            set: [(0,0);3]
        };

        assert_eq!( game.add_point(Player::Home), FullGame  {
            game: ((Player::Home, 0) ,(Player::Oponent, 0)),
            stage: Stage::Deuce,
            set: [(0,0);3]
        });
    }
    #[test]
    fn check_game_win() {
        let mut game = FullGame  {
            game: ((Player::Home, 30) ,(Player::Oponent, 40)),
            stage: Stage::Normal,
            set: [(0,0);3]
        };

        assert_eq!( game.add_point(Player::Oponent), FullGame  {
            game: ((Player::Home, 0) ,(Player::Oponent, 0)),
            stage: Stage::Normal,
            set: [(0,1), (0,0),(0,0)]
        });
    }

    #[test]
    fn check_second_set_win() {
        let mut game = FullGame  {
            game: ((Player::Home, 30) ,(Player::Oponent, 40)),
            stage: Stage::Normal,
            set: [(7,5), (0,0),(0,0)]
        };

        assert_eq!( game.add_point(Player::Oponent), FullGame  {
            game: ((Player::Home, 0) ,(Player::Oponent, 0)),
            stage: Stage::Normal,
            set: [(7,5), (0,1),(0,0)]
        });
    }

}