use super::{Agent, Game, Turn::*};

use std::iter;

use rand::Rng;

pub struct RandomAgent;

impl Default for RandomAgent {
    fn default() -> Self {
        Self
    }
}

impl Agent for RandomAgent {
    fn pick_index(&mut self, game: &Game) -> usize {
        let player = if game.turn() == Player1 {
            &game.player1
        } else {
            &game.player2
        };

        iter::repeat_with(|| rand::thread_rng().gen_range(0..16))
            .find(|&index| player.is_valid_index(index))
            .expect("No valid index?")
    }
}
