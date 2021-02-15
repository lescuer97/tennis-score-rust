

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
pub struct Game  {
   pub player1: u8,
    pub player2: u8,
    pub stage: Stage
}

impl  Game {
    // creates a new game
  pub  fn new() -> Self  {
        Game {
            player1: 0,
            player2: 0,
            stage: Stage::Normal,
        }
    }

    pub fn add_point_player_1(&mut self ) -> Self {
// TODO ADD THE CORRECT AMOUNT OF POINTS
// TODO CHECK IF THERE IS A DEUCE CHECK WHEN IT'S WON
// TODO IF THERE IS TIE BREAK CHECK IF THERE IS ADVANTAGE OF TWO POINT AND 7 OR OVER

// FIXME change the way that points get added so it returns correct values
//  match self.player1 {
//         40 if self.player2 < 40 => return Self{ player1: 0, player2: 0, stage: stage: Stage::Normal},
//         0 => return Self{ player1: self.player1 += 15, player2: self.player2, stage: stage: Stage::Normal},
//         15 => return Self{ player1: self.player1 += 15, player2: self.player2, stage: stage: Stage::Normal},
//         30 => return Self{ player1: self.player1 += 10, player2: self.player2, stage: stage: Stage::Normal},
//         _ => Self{ player1: self.player1, player2: self.player2, stage: stage: Stage::Normal}
//     }

  
    
    let game = compare_deuce(self);

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