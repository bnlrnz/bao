use super::{Agent, Game, Turn::*};

use std::iter;

use radiate::Neat;

pub struct RadiateAgent<'o> {
    model: &'o mut Neat,
    input: [f32; 33],
    output: Vec<f32>,
    indexed_output: [(usize, f32); 16],
}

impl<'o> RadiateAgent<'o> {
    pub fn new(model: &'o mut Neat) -> Self {
        Self {
            model,
            input: [0.0; 33],
            output: vec![0.0; 16],
            indexed_output: [(0, 0.0); 16],
        }
    }
}

impl Agent for RadiateAgent<'_> {
    fn pick_index(&mut self, game: &Game) -> usize {
        let (player, opponent) = if game.turn() == Player1 {
            (&game.player1, &game.player2)
        } else {
            (&game.player2, &game.player1)
        };

        for (src, dst) in player
            .board_half
            .iter()
            .chain(opponent.board_half.iter())
            .map(|&val| val as f32)
            .chain(iter::once(game.direction.input_enc()))
            .zip(&mut self.input[..])
        {
            *dst = src;
        }

        self.output = self
            .model
            .forward(&self.input.to_vec())
            .expect("No output?");

        for (src, dst) in self
            .output
            .iter()
            .copied()
            .enumerate()
            .zip(&mut self.indexed_output)
        {
            *dst = src;
        }

        self.indexed_output
            .sort_unstable_by(|(_, a), (_, b)| b.partial_cmp(a).expect("NaN?"));

        // Select the best index that is valid.
        self.indexed_output
            .iter()
            .find(|&&(index, _)| player.is_valid_index(index))
            .expect("No valid index?")
            .0
    }
}
