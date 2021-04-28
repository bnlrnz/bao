mod bao;
use bao::*;

use std::usize;
use std::sync::Arc;
use std::error::Error;

extern crate radiate;
use radiate::prelude::*;

fn random_ai_game(){
    let mut first: usize = 0;
    let mut second: usize = 0;
    for _ in 0..100000 {
        let mut game = Game::new(
            Direction::CW,
            Mode::Easy,
            Player::new("Player 1", PlayerAgent::AiRandom),
            Player::new("Player 2", PlayerAgent::AiRandom),
        );

        match game.run().1 {
            PlayerPosition::First => {first+=1}
            PlayerPosition::Second => {second+=1}
        }
    }

    println!("First Player: {}", first);
    println!("Second Player: {}", second);
}

impl Problem<Neat> for Game {
    fn empty() -> Self {
        Game::new(
            Direction::CW,
            Mode::Easy,
            Player::new("Player 1", PlayerAgent::AiRandom),
            Player::new("Player 2", PlayerAgent::AiRandom),
        )
    }

    fn solve(&self, member: &mut Neat) -> f32 {
        /*let mut input = vec![];
        for b in self.player1.read_board(){
            input.push(*b as f32);
        }
        for b in self.player2.read_board(){
            input.push(*b as f32);
        }
        input.push(if matches!(self.direction, Direction::CW) {1.0} else {0.0});

        self.player2.set_choose_bowl_index(Box::new(||{

            let guesses = member.forward(&input).expect("Could not get guesses");


            let mut iter = guesses.iter().enumerate();
            let init = iter.next().ok_or("Need at least one input").unwrap();
            let index = iter.try_fold(init, |acc, x|{
                let cmp = x.1.partial_cmp(acc.1)?;
                let max = if let std::cmp::Ordering::Greater = cmp {
                    x
                } else {
                    acc
                };
                Some(max)
            });
            
            index.unwrap().0
        }));
        */

        self.player2.set_choose_bowl_index(Box::new(||{
            1
        }));

        return match self.run().1 {
            PlayerPosition::First => {0.0}
            PlayerPosition::Second => {1.0}
        }
    }
}


fn train() -> Result<(), Box<dyn Error>>{
    let mut neat_env = NeatEnvironment::new()
        .set_input_size(33)
        .set_output_size(16)
        .set_weight_mutate_rate(0.8)
        .set_edit_weights(0.1)
        .set_activation_functions(vec![
            Activation::Sigmoid,
            Activation::Relu
        ]);

        let starting_net = Neat::base(&mut neat_env);
        let num_evolve = 200;
        
        let (mut solution, _) = Population::<Neat, NeatEnvironment, Game>::new()
            .constrain(neat_env)
            .size(200)
            .populate_clone(starting_net)
            .debug(true)
            .dynamic_distance(true)
            .configure(Config {
                inbreed_rate: 0.001,
                crossover_rate: 0.75,
                distance: 0.5,
                species_target: 5
            })
            .stagnation(15, vec![
                Genocide::KillWorst(0.9)
            ])
            .run(|_, fit, num| {
                println!("Generation: {} score: {}", num, fit);
                num == num_evolve
            })?;
        
        println!("{:#?}", solution);
        solution.save("ai.json")?;
        Ok(())
}

fn main() -> Result<(), Box<dyn Error>>{
    let param: String = std::env::args().skip(1).take(1).collect();

    if param == "random" {
        random_ai_game();
        return Ok(());
    }

    if param == "train" {
        train();
        return Ok(());
    }

    Ok(())
}
