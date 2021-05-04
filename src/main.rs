mod bao;
use bao::{
    Direction, Game, HumanAgent, MaximizeAgent, Mode, Player, RadiateAgent, RandomAgent,
    RustNeatAgent,
};

use radiate::prelude::*;
use radiate::{Neat, NeatEnvironment, Problem};
use rustneat::{Environment, Organism, Population};

use std::fs::OpenOptions;
use std::io::prelude::*;

fn random_ai_game() {
    let mut results = [0; 2];

    for _ in 0..1000000 {
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
        let mut fitness = 0.0;

        for _ in 0..100 {
            fitness += if Game::new(
                Direction::CW,
                Mode::Easy,
                Player::new("Player 1", 0),
                Player::new("Player 2", 1),
            )
            .play(
                &mut MaximizeAgent::default(),
                &mut RustNeatAgent::new(organism),
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
        fitness
    }
}

fn train_rustneat() {
    let mut population = Population::create_population(100);
    let mut environment = GameEnvironment;
    let mut champion: Option<Organism> = None;

    while champion.is_none() {
        population.evolve();
        population.evaluate_in(&mut environment);
        for organism in &population.get_organisms() {
            println!("Fitness: {}", organism.fitness);
            if organism.fitness > 90.0 {
                champion = Some(organism.clone());
            }
        }
    }

    println!("{:?}", champion.unwrap().genome);
}

impl Problem<Neat> for Game {
    fn empty() -> Self {
        Game::new(
            Direction::CW,
            Mode::Easy,
            Player::new("Player 1", 0),
            Player::new("Player 2", 1),
        )
    }

    fn solve(&self, member: &mut Neat) -> f32 {
        let mut fitness = 0.0;

        let runs = 100;
        for _ in 0..runs {
            let result = Game::new(
                Direction::CW,
                Mode::Easy,
                Player::new("Player 1", 0),
                Player::new("Player 2", 1),
            )
            .play(&mut RandomAgent::default(), &mut RadiateAgent::new(member));
            // println!("{:?} won!", result.winner);
            // println!("{:?} lost!", result.loser);
            // println!("=================");
            fitness += if result.winner.tag() == 1 { 1.0 } else { -1.0 }
        }

        fitness / runs as f32
    }
}

fn train_radiate() {
    let mut neat_env = NeatEnvironment::new()
        .set_input_size(33)
        .set_output_size(16)
        .set_weight_mutate_rate(0.5)
        .set_edit_weights(0.5)
        .set_weight_perturb(1.0)
        .set_new_node_rate(0.5)
        .set_new_edge_rate(0.5)
        .set_reactivate(0.5)
        .set_activation_functions(vec![
            Activation::Tanh,
            Activation::Relu,
            Activation::Sigmoid,
        ]);

    let starting_net = Neat::base(&mut neat_env);
    let (solution, _) = radiate::Population::<Neat, NeatEnvironment, Game>::new()
        .constrain(neat_env)
        .size(500)
        .populate_clone(starting_net)
        .debug(true)
        .dynamic_distance(true)
        .configure(Config {
            inbreed_rate: 0.001,
            crossover_rate: 0.9,
            distance: 0.75,
            species_target: 5,
        })
        .stagnation(20, vec![Genocide::KillWorst(0.9)])
        .survivor_criteria(radiate::SurvivalCriteria::Fittest)
        .parental_criteria(radiate::ParentalCriteria::BestInSpecies)
        .run(|_, fit, num| {
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open("log.txt")
                .unwrap();
            writeln!(file, "Generation: {} score: {}", num, fit).expect("could not write log");
            fit > 0.95
        })
        .expect("radiate could not run or crashed");

    //println!("{:#?}", solution);
    solution
        .save("radiate_ai.json")
        .expect("Could not write ai file");
}

fn main() {
    let param: String = std::env::args().skip(1).take(1).collect();

    if param == "random" {
        random_ai_game();
    }

    if param == "human" {
        human_game();
    }

    if param == "rustneat" {
        train_rustneat();
    }

    if param == "radiate" {
        train_radiate();
    }
}
