mod bao;
use bao::*;

use std::sync::{Arc, Mutex};
use std::usize;

extern crate rustneat;
use rustneat::Environment;
use rustneat::Organism;
use rustneat::Population;

fn random_ai_game() {
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
            PlayerPosition::First => first += 1,
            PlayerPosition::Second => second += 1,
        }
    }

    println!("First Player: {}", first);
    println!("Second Player: {}", second);
}

struct GameEnvironment;

impl Environment for GameEnvironment {
    fn test(&self, organism: &mut Organism) -> f64 {
        let mut fitness = 0.0;

        for _ in 0..100 {
            let mut game = Game::new(
                Direction::CW,
                Mode::Easy,
                Player::new("Player 1", PlayerAgent::AiRandom),
                Player::new("Player 2", PlayerAgent::AiTraining),
            );

            let org = Arc::new(Mutex::new(organism.clone()));
            game.player2
                .set_choose_bowl_index(Box::new(move |own, opp, dir| {
                    let output: [f64; 16] = [0.0; 16];
                    let mut input: Vec<f64> = Vec::new();

                    for b in own {
                        input.push(*b as f64);
                    }
                    for b in opp {
                        input.push(*b as f64);
                    }
                    input.push(match dir {
                        Direction::CW => 0.0,
                        Direction::CCW => 1.0,
                    });

                    org.lock().unwrap().activate(&input, &mut output.to_vec());

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

                    let index = result.unwrap().0;

                    if own[index] < 2 {
                        // ai should lose the game if it chooses an invalid index!
                        return 42;
                    }

                    index
                }));

            match game.run().1 {
                PlayerPosition::First => {
                    fitness += 0.0;
                }
                PlayerPosition::Second => {
                    fitness += 1.0;
                }
            };
        }

        println!("Fitness: {}", fitness);
        fitness
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
            if organism.fitness > 75.0 {
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
