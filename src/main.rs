mod bao;
use Box::new;
use bao::*;

use std::usize;

extern crate rustneat;
use rustneat::Environment;
use rustneat::Organism;
use rustneat::Population;

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

struct GameEnvironment;

impl Environment for GameEnvironment {
    fn test(&self, organism: &mut Organism) -> f64 {
        let mut game = Game::new(
            Direction::CW,
            Mode::Easy,
            Player::new("Player 1", PlayerAgent::AiRandom),
            Player::new("Player 2", PlayerAgent::AiRandom),
        );

        game.player2.set_choose_bowl_index(Box::new(|own, opp, dir|{
            let mut output = vec![16f64];
            let mut input: Vec<f64> = Vec::new();

            for b in own{
                input.push(*b as f64);
            }
            for b in opp{
                input.push(*b as f64);
            }
            input.push(match dir {
                Direction::CW => {0.0}
                Direction::CCW => {1.0}
            });

            organism.activate(input, output);

                // Use enumerate to get the index
            let mut iter = output.iter().enumerate();
            // we get the first entry
            let init = iter.next().ok_or("Need at least one input").unwrap();
            // we process the rest
            let result = iter.try_fold(init, |acc, x| {
                // return None if x is NaN
                let cmp = x.1.partial_cmp(acc.1)?;
                // if x is greater the acc
                let max = if let std::cmp::Ordering::Greater = cmp {
                    x
                } else {
                    acc
                };
                Some(max)
            });

            result.unwrap().0
        }));

        match game.run().1 {
            PlayerPosition::First => {0.0}
            PlayerPosition::Second => {1.0}
        }
    }
}

fn train() {
    let mut population = Population::create_population(100);
    let mut environment = GameEnvironment;
    let mut champion: Option<Organism> = None;

    while champion.is_none() {
        population.evolve();
        population.evaluate_in(&mut environment);
        for organism in &population.get_organisms() {
            println!("Fitness: {}", organism.fitness);
            if organism.fitness > 10.0 {
                champion = Some(organism.clone());
            }
        }
    }

    println!("{:?}", champion.unwrap().genome);
}

fn main() {
    let param: String = std::env::args().skip(1).take(1).collect();

    if param == "random" {
        random_ai_game();
    }

    if param == "train" {
        train();
    }
}
