use super::{Agent, Game, MoveResult, Turn::*};

pub struct MaximizeAgent;

impl Default for MaximizeAgent {
    fn default() -> Self {
        Self
    }
}

impl Agent for MaximizeAgent {
    fn pick_index(&mut self, game: &Game) -> usize {
        let (mut player, mut opponent) = if game.turn() == Player1 {
            (game.player1.clone(), game.player2.clone())
        } else {
            (game.player2.clone(), game.player1.clone())
        };

        let mut max_steal_index = 0;
        let mut max_steal = 0;
        for index in 0..16 {
            let steal = match Game::steal_dry_run(
                index,
                game.direction,
                game.mode,
                &mut player,
                &mut opponent,
            ) {
                MoveResult::None(s) => s,
                MoveResult::Lost(s) => s,
                MoveResult::Won(s) => s,
            };

            if steal > max_steal {
                max_steal = steal;
                max_steal_index = index;
            }
        }

        max_steal_index
    }
}
