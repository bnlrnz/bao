use std::io;
use std::print;

#[derive(Copy, Clone)]
#[allow(unused)]
pub enum Direction {
    CW,
    CCW,
}

#[derive(Copy, Clone)]
#[allow(unused)]
pub enum Mode {
    NORMAL, // all stones required
    EASY,   // just the inner row must be empty to win
}

#[derive(Copy, Clone, PartialEq)]
#[allow(unused)]
pub enum PlayerAgent {
    HUMAN,
    AI,
}

pub enum PlayerPosition {
    FIRST,
    SECOND,
}

pub struct Player {
    name: &'static str,
    agent: PlayerAgent,
    board_half: [u8; 16],
    pub choose_bowl_index: Box<dyn Fn() -> usize>,
}

impl Player {
    pub fn new(name: &'static str, agent: PlayerAgent) -> Self {
        Self {
            name,
            agent,
            board_half: [2; 16],
            choose_bowl_index: Box::new(Self::read_index),
        }
    }

    #[allow(unused)]
    pub fn set_choose_bowl_index(&mut self, func: Box<dyn Fn() -> usize>) {
        self.choose_bowl_index = func;
    }

    #[allow(unused)]
    pub fn read_board(&self) -> &[u8; 16] {
        &self.board_half
    }

    fn read_index() -> usize {
        let mut index: Option<usize> = None;
        while index == None {
            let mut input_text = String::new();
            io::stdin()
                .read_line(&mut input_text)
                .expect("failed to read from stdin");

            let trimmed = input_text.trim();
            match trimmed.parse::<usize>() {
                Ok(i) => return i,
                Err(..) => {
                    println!("Please enter a valid number!");
                    index = None
                }
            };
        }
        index.unwrap()
    }
}

pub struct Game {
    pub direction: Direction,
    pub mode: Mode,
    turn: usize,
    player1: Player,
    player2: Player,
}

impl Game {
    pub fn new(direction: Direction, mode: Mode, player1: Player, player2: Player) -> Self {
        Self {
            direction,
            mode,
            turn: 1,
            player1,
            player2,
        }
    }

    #[allow(unused)]
    pub fn run(&mut self) {
        while self.move_possible() && !self.game_over() {
            if self.get_current_player().agent == PlayerAgent::HUMAN {
                self.print_board();
            }
            self.make_move(self.pick_index());
            self.next_turn();
        }

        println!(
            "Congratulation {}. You won!",
            self.get_winner().0.name
        );
    }

    #[inline(always)]
    pub fn get_mut_current_and_opponent_player(&mut self) -> (&mut Player, &mut Player) {
        match self.turn % 2 {
            1 => (&mut self.player1, &mut self.player2),
            0 => (&mut self.player2, &mut self.player1),
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    pub fn next_turn(&mut self) {
        self.turn += 1;
    }

    #[inline(always)]
    pub fn current_player_position(&self) -> PlayerPosition {
        match self.turn % 2 {
            1 => PlayerPosition::FIRST,
            0 => PlayerPosition::SECOND,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    pub fn get_current_player(&self) -> &Player {
        match self.current_player_position() {
            PlayerPosition::FIRST => &self.player1,
            PlayerPosition::SECOND => &self.player2,
        }
    }

    #[inline(always)]
    pub fn get_opponent_player(&self) -> &Player {
        match self.current_player_position() {
            PlayerPosition::SECOND => &self.player1,
            PlayerPosition::FIRST => &self.player2,
        }
    }

    #[allow(unused)]
    fn pick_index(&self) -> usize {
        let player = self.get_current_player();

        let mut index = 0;
        let mut valid_index = false;
        while !valid_index {
            if player.agent == PlayerAgent::HUMAN {
                println!("{} enter bowl index: ", player.name);
            }

            // determine which bowl to play
            index = (player.choose_bowl_index)();

            if (0..16).contains(&index) && player.board_half[index] >= 2 {
                valid_index = true;
            } else {
                if player.agent == PlayerAgent::HUMAN {
                    println!(
                        "Please enter a valid index (0-15). Bowl must contain at least 2 stones."
                    );
                }
            }
        }
        index
    }

    pub fn make_move(&mut self, start_index: usize){
        let mut index = start_index;
        let direction = self.direction;
        let mode = self.mode;

        let (player, opponent) = self.get_mut_current_and_opponent_player();

        let mut hand: u8 = player.board_half[index];
        player.board_half[index] = 0;

        //println!("{} choose index {}", player.name, index);

        while hand > 0 {
            index = Self::next_index(index, direction);
            hand -= 1;
            player.board_half[index] += 1;

            if hand == 0 && player.board_half[index] >= 2 {
                hand = player.board_half[index];
                player.board_half[index] = 0;

                // steal from opponent
                if (8..=15).contains(&index) {
                    let opponent_index = (15 - index) + 8;

                    hand += match mode {
                        Mode::EASY => {
                            let steal = opponent.board_half[opponent_index];
                            opponent.board_half[opponent_index] = 0;
                            steal
                        }
                        Mode::NORMAL => {
                            let opponent_2nd_index = 15 - opponent_index;
                            let steal = opponent.board_half[opponent_index]
                                + opponent.board_half[opponent_2nd_index];
                            opponent.board_half[opponent_index] = 0;
                            opponent.board_half[opponent_2nd_index] = 0;
                            steal
                        }
                    };
                    
                    // check win condition after steal!
                    if Self::check_player_lost(mode, opponent){
                        return;
                    }
                }
            }
        }
    }

    #[inline(always)]
    fn check_player_lost(mode: Mode, player: &Player) -> bool {
        match mode {
            Mode::EASY => {
                for bowl in player.board_half.iter().take(16).skip(8) {
                    if bowl != &0 {
                        return false;
                    }
                }
                true
            }
            Mode::NORMAL => {
                for bowl in player.board_half.iter().take(16) {
                    if bowl != &0 {
                        return false;
                    }
                }
                true
            }
        }
    }

    #[allow(unused)]
    #[inline(always)]
    pub fn get_winner(&self) -> (&Player, PlayerPosition) {
        if Self::check_player_lost(self.mode, &self.player1) {
            return (&self.player2, PlayerPosition::SECOND);
        }
        (&self.player1, PlayerPosition::FIRST)
    }

    #[inline(always)]
    fn next_index(index: usize, direction: Direction) -> usize {
        match direction {
            Direction::CW => {
                if index < 1 {
                    15
                } else {
                    index - 1
                }
            }
            Direction::CCW => {
                if index + 1 > 15 {
                    0
                } else {
                    index + 1
                }
            }
        }
    }

    pub fn print_board(&self) {
        println!(
            "           {:2}Player 2{:2}",
            if self.turn % 2 == 0 { "->" } else { "" },
            if self.turn % 2 == 0 { "<-" } else { "" },
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
            self.turn
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
            if self.turn % 2 == 1 { "->" } else { "" },
            if self.turn % 2 == 1 { "<-" } else { "" },
        );
    }

    pub fn move_possible(&self) -> bool {
        for bowl in self.get_current_player().board_half.iter() {
            if bowl >= &2 {
                return true;
            }
        }
        if self.get_current_player().agent == PlayerAgent::HUMAN {
            println!(
                "{}: no possible moveds left :(",
                self.get_current_player().name
            );
        }
        false
    }

    pub fn game_over(&self) -> bool {
        let current_player_board = self.get_current_player().board_half;

        match self.mode {
            Mode::EASY => {
                for bowl in current_player_board.iter().take(16).skip(8) {
                    if bowl != &0 {
                        return false;
                    }
                }
                true
            }
            Mode::NORMAL => {
                for bowl in current_player_board.iter().take(16) {
                    if bowl != &0 {
                        return false;
                    }
                }
                true
            }
        }
    }
}
