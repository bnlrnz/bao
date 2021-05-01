mod bao;
use bao::*;

use rustneat::{Environment, Organism, Population};

fn random_ai_game() {
    let mut results = [0; 2];

    for _ in 0..100000 {
        let winner_tag = Game::new(
            Direction::CW,
            Mode::Easy,
            Player::new("Player 1", 0),
            Player::new("Player 2", 1),
        )
        .play(&mut RandomAgent::default(), &mut RandomAgent::default())
        .winner
        .tag();

        results[winner_tag] += 1;
    }

    println!("First Player: {}", results[0]);
    println!("Second Player: {}", results[1]);
}

fn human_game() {
    let winner = Game::new(
        Direction::CW,
        Mode::Easy,
        Player::new("Player 1", 0),
        Player::new("Player 2", 1),
    )
    .play(&mut HumanAgent::default(), &mut RandomAgent::default())
    .winner;

    println!("Winner: {}", winner.name());
}

struct GameEnvironment;

impl Environment for GameEnvironment {
    fn test(&self, organism: &mut Organism) -> f64 {
        if Game::new(
            Direction::CW,
            Mode::Easy,
            Player::new("Player 1", 0),
            Player::new("Player 2", 1),
        )
        .play(
            &mut RandomAgent::default(),
            &mut TrainingAgent::new(organism),
        )
        .winner
        .tag()
            == 1
        {
            1.0
        } else {
            0.0
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

    if param == "human" {
        human_game();
    }

    if param == "train" {
        train();
    }
}
