

 fn compare_deuce(game: &mut Game) -> Game {
    match game {
        // verifies if there is a deuce
        game if game.player1 == 40 && game.player2 == 40 && game.stage == Stage::Normal => return Game {
            player1: 0,
            player2: 0,
            stage: Stage::Deuce,
        },
        _ => return Game {player1: game.player1,player2: game.player2,stage: game.stage}
    }
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

    return game;
    }
}

#[derive(PartialEq, Debug,Clone, Copy)]
pub enum Stage {
    Normal,
    Deuce,
    TieBreak,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn initialize(){
     
        assert_eq!(Game::new(), Game {
            player1: 0,
            player2: 0,
            stage: Stage::Normal,
        })
    }
    #[test]
    fn add_point_test() {
        let mut game = Game  {
            player1: 0,
            player2: 0,
            stage: Stage::Normal,
        };

        assert_eq!( game.add_point_player_1(), Game  {
            player1: 15,
            player2: 0,
            stage: Stage::Normal,
        });
    }

}