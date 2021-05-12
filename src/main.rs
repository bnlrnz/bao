mod bao;
use bao::{Direction, Game, HumanAgent, MaximizeAgent, Mode, Player, RadiateAgent, RandomAgent};

use radiate::prelude::*;
use radiate::{Neat, NeatEnvironment, Problem};

use std::fs::OpenOptions;
use std::io::prelude::*;

fn random_ai_game() {
    let mut neat = Neat::load("radiate_ai_v_ai.json").expect("Could not load ai file");

    let mut results = [0; 2];
    for _ in 0..100000 {
        let winner_tag = Game::new(
            Direction::CW,
            Mode::Easy,
            Player::new("Player 1", 0),
            Player::new("Player 2", 1),
        )
        .play(&mut RandomAgent::default(), &mut RadiateAgent::new(&mut neat))
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
        //let mut neat = Neat::load("radiate_ai_final2.json").expect("Could not load ai file");

        let mut fitness = 0.0;

        let runs = 100;
        for _ in 0..runs {
            let mut radiate_agent = RadiateAgent::new(member);
            // let result = Game::new(
            //     Direction::CW,
            //     Mode::Easy,
            //     Player::new("Player 1", 0),
            //     Player::new("Player 2", 1),
            // )
            // .play(&mut MaximizeAgent::default(), &mut radiate_agent);
            // //  println!("{:?} won!", result.winner);
            // //  println!("{:?} lost!", result.loser);
            // //  println!("=================");
            // fitness += if result.winner.tag() == 1 { 1.0 } else { -1.0 };
            // let result = Game::new(
            //     Direction::CW,
            //     Mode::Easy,
            //     Player::new("Player 1", 0),
            //     Player::new("Player 2", 1),
            // )
            // .play(&mut radiate_agent, &mut MaximizeAgent::default());
            // //  println!("{:?} won!", result.winner);
            // //  println!("{:?} lost!", result.loser);
            // //  println!("=================");
            // fitness += if result.winner.tag() == 0 { 1.0 } else { -1.0 };


            // let result = Game::new(
            //     Direction::CW,
            //     Mode::Easy,
            //     Player::new("Player 1", 0),
            //     Player::new("Player 2", 1),
            // )
            // .play(&mut RadiateAgent::new(&mut neat), &mut radiate_agent);
            // // println!("{:?} won!", result.winner);
            // // println!("{:?} lost!", result.loser);
            // // println!("=================");
            // fitness += if result.winner.tag() == 1 { 1.0 } else { -1.0 };
            // let result = Game::new(
            //     Direction::CW,
            //     Mode::Easy,
            //     Player::new("Player 1", 0),
            //     Player::new("Player 2", 1),
            // )
            // .play( &mut radiate_agent, &mut RadiateAgent::new(&mut neat));
            // // println!("{:?} won!", result.winner);
            // // println!("{:?} lost!", result.loser);
            // // println!("=================");
            // fitness += if result.winner.tag() == 0 { 1.0 } else { -1.0 };



            let result = Game::new(
                Direction::CW,
                Mode::Easy,
                Player::new("Player 1", 0),
                Player::new("Player 2", 1),
            )
            .play(&mut RandomAgent::default(), &mut radiate_agent);
            // println!("{:?} won!", result.winner);
            // println!("{:?} lost!", result.loser);
            // println!("=================");
            fitness += if result.winner.tag() == 1 { 1.0 } else { -1.0 };
            let result = Game::new(
                Direction::CW,
                Mode::Easy,
                Player::new("Player 1", 0),
                Player::new("Player 2", 1),
            )
            .play(&mut radiate_agent, &mut RandomAgent::default());
            // println!("{:?} won!", result.winner);
            // println!("{:?} lost!", result.loser);
            // println!("=================");
            fitness += if result.winner.tag() == 0 { 1.0 } else { -1.0 };
        }
        fitness / (runs * 2) as f32
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

    let mut neat = Neat::load("radiate_ai_final3.json").expect("Could not load ai file");


    let target_gen = 5000;
    let starting_net = Neat::base(&mut neat_env);
    let (solution, _) = radiate::Population::<Neat, NeatEnvironment, Game>::new()
        .constrain(neat_env)
        .size(1000)
        .populate_clone(neat)
        .debug(true)
        .dynamic_distance(true)
        .configure(Config {
            inbreed_rate: 0.001,
            crossover_rate: 0.9,
            distance: 0.75,
            species_target: 15,
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
            fit > 0.90 || num == target_gen
        })
        .expect("radiate could not run or crashed");

    //println!("{:#?}", solution);
    solution
        .save("radiate_ai_v_ai.json")
        .expect("Could not write ai file");
}

fn main() {
    let param: String = std::env::args().skip(1).take(1).collect();

    if param == "radiate" {
        train_radiate();
    }

    if param == "random" {
        random_ai_game();
    }

    if param == "human" {
        human_game();
    }
}
