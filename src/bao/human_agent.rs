use super::{Agent, Game, Turn::*};

use std::io;

pub struct HumanAgent;

impl Default for HumanAgent {
    fn default() -> Self {
        Self
    }
}

impl Agent for HumanAgent {
    fn pick_index(&mut self, game: &Game) -> usize {
        game.print_board();

        let player = if game.turn() == Player1 {
            &game.player1
        } else {
            &game.player2
        };

        loop {
            println!("{}, enter bowl index: ", player.name);
            let mut input_text = String::new();

            io::stdin()
                .read_line(&mut input_text)
                .expect("failed to read from stdin");

            let trimmed = input_text.trim();

            let index = match trimmed.parse::<usize>() {
                Ok(i) => i,
                Err(_) => {
                    println!("Please enter a valid index between 0 and 15.");
                    continue;
                }
            };

            if !player.is_valid_index(index) {
                println!("Bowl must contain at least 2 stones.");
                continue;
            }

            return index;
        }
    }
}

impl Game {
    fn print_board(&self) {
        println!(
            "           {:2}Player 2{:2}",
            if self.turn() == Player2 { "->" } else { "" },
            if self.turn() == Player2 { "<-" } else { "" },
        );
        print!("|");
        for i in (0..8).rev() {
            print!(" {:2} |", i);
        }
        println!();
        println!("-----------------------------------------");
        print!("|");
        for i in (0..8).rev() {
            print!(" {:2} |", self.player2.board_half[i]);
        }
        println!();

        println!("-----------------------------------------");
        print!("|");
        for i in 8..16 {
            print!(" {:2} |", i);
        }
        println!();
        println!("-----------------------------------------");
        print!("|");
        for i in 8..16 {
            print!(" {:2} |", self.player2.board_half[i]);
        }
        println!(" Stones: {}", self.player2.board_half.iter().sum::<u8>());
        println!(
            "==================================================== Round: {}",
            self.turn_count
        );
        print!("|");
        for i in (8..16).rev() {
            print!(" {:2} |", self.player1.board_half[i]);
        }
        println!(" Stones: {}", self.player1.board_half.iter().sum::<u8>());
        println!("-----------------------------------------");
        print!("|");
        for i in (8..16).rev() {
            print!(" {:2} |", i);
        }
        println!();
        println!("-----------------------------------------");
        print!("|");
        for i in 0..8 {
            print!(" {:2} |", self.player1.board_half[i]);
        }
        println!();
        println!("-----------------------------------------");
        print!("|");
        for i in 0..8 {
            print!(" {:2} |", i);
        }
        println!();
        println!(
            "           {:2}Player 1{:2}",
            if self.turn() == Player1 { "->" } else { "" },
            if self.turn() == Player1 { "<-" } else { "" },
        );
    }
}
