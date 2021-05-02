use super::{Agent, Game, Turn::*};

use std::iter;

use rustneat::Organism;

pub struct RustNeatAgent<'o> {
    organism: &'o mut Organism,
    input: [f64; 33],
    output: Vec<f64>,
    indexed_output: [(usize, f64); 16],
}

impl<'o> RustNeatAgent<'o> {
    pub fn new(organism: &'o mut Organism) -> Self {
        Self {
            organism,
            input: [0.0; 33],
            output: vec![0.0; 16], // must be a vec because of activate()
            indexed_output: [(0, 0.0); 16],
        }
    }
}

impl Agent for RustNeatAgent<'_> {
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
            .map(|&val| val as f64)
            .chain(iter::once(game.direction.input_enc() as f64))
            .zip(&mut self.input[..])
        {
            *dst = src;
        }

        self.organism.activate(&self.input, &mut self.output);

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
