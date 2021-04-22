mod bao;
use bao::*;
use neat_core::{Configuration, Network, NEAT};
use neat_environment::Environment;
use neat_export::to_file;

use rand::{seq::index::IndexVecIter, Rng};

impl Environment for Game {
    type State = ([u8; 16], [u8; 16], Direction);
    type Input = usize;

    fn state(&self) -> Self::State {
        (
            *self.get_current_player().read_board(),
            *self.get_opponent_player().read_board(),
            self.direction,
        )
    }

    fn step(&mut self, input: Self::Input) -> Result<(), ()> {
        let index = match self.current_player_position() {
            // this one plays random
            PlayerPosition::FIRST => {
                let (player, _) = self.get_mut_current_and_opponent_player();

                let mut index = 0;
                let mut valid_index = false;
                while !valid_index {
                    let mut rng = rand::thread_rng();
                    index = rng.gen_range(0..16);

                    if (0..16).contains(&index) && player.read_board()[index] >= 2 {
                        valid_index = true;
                    }
                }

                index
            }
            // this is the "AI"
            PlayerPosition::SECOND => {
                if !(0..=15).contains(&input) {
                    return Err(());
                }

                let (player, _) = self.get_mut_current_and_opponent_player();

                if player.read_board()[input] < 2 {
                    return Err(());
                }

                input
            }
        };

        self.make_move(index);
        self.next_turn();
        Ok(())
    }

    fn done(&self) -> bool {
        !self.move_possible() || self.game_over()
    }

    fn reset(&mut self) {
        *self = Game::new(
            Direction::CW,
            Mode::EASY,
            Player::new("Player 1", PlayerAgent::AI),
            Player::new("Player 2", PlayerAgent::AI),
        );
    }

    fn render(&self) {
        self.print_board();
    }

    fn fitness(&self) -> f64 {
        match self.get_winner().1 {
            PlayerPosition::FIRST => 0.,
            PlayerPosition::SECOND => 1.,
        }
    }
}

fn state_to_inputs(env: &Game) -> Vec<f64> {
    let (p1, p2, direction) = env.state();

    let mut inputs: Vec<f64> = Vec::new();

    for b in p1.iter() {
        inputs.push(f64::from(*b))
    }

    for b in p2.iter() {
        inputs.push(f64::from(*b))
    }

    inputs.push(if matches!(direction, Direction::CCW) {
        1.0
    } else {
        0.0
    });

    inputs
}

fn move_from_outputs(outputs: &[f64]) -> usize {
    outputs
        .iter()
        .enumerate()
        .fold((0, -999.), |(max_index, max_output), (index, output)| {
            if output > &max_output {
                (index, *output)
            } else {
                (max_index, max_output)
            }
        })
        .0
}

fn main() {
    let mut system = NEAT::new(33, 16, |network| {
        let games = 100;
        let mut games_won = 0;

        let mut env = Game::new(
            Direction::CW,
            Mode::EASY,
            Player::new("Player 1", PlayerAgent::AI),
            Player::new("Player 2", PlayerAgent::AI),
        );

        for _ in 0..games {
            env.reset();

            while !env.done() {
                let inputs = state_to_inputs(&env);
                let mut outputs: Vec<f64> = network.forward_pass(inputs.clone()).clone();
                let mut index = move_from_outputs(&outputs);

                while env.step(index).is_err() {
                    outputs[index] = -999.0;
                    index = move_from_outputs(&outputs);
                }
            }

            games_won += match env.get_winner().1 {
                PlayerPosition::FIRST => 0,
                PlayerPosition::SECOND => 1,
            }
        }
        println!("Won {}/{}", games_won, games);
        games_won as f64 / games as f64
    });

    system.set_configuration(Configuration {
        max_generations: 500,
        population_size: 50,
        node_cost: 0.001,
        connection_cost: 0.0005,
        compatibility_threshold: 3.0,
        ..Default::default()
    });

    system.add_hook(1, |i, system| {
        let (_, _, fitness) = system.get_best();
        println!("Generation {}, best fitness is {}", i, fitness);
    });

    let (network, fitness) = system.start();

    println!(
        "Found network with {} nodes and {} connections, of fitness {}",
        network.nodes.len(),
        network.connections.len(),
        fitness
    );

    to_file("network.bin", &network);
}
