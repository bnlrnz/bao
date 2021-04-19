use core::panic;
use std::print;
use std::io;

#[derive(Copy, Clone)]
enum Direction {
    CW,
    CCW,
}
#[derive(Copy, Clone)]
enum Mode {
    NORMAL,// all stones required
    EASY,  // just the inner row must be empty to win
}

#[derive(Copy, Clone, PartialEq)]
enum PlayerAgent{
    HUMAN,
    AI,
}

struct Player{
    name: &'static str,
    agent: PlayerAgent,
    board_half: [u8; 16],
}

impl Player{
    fn new(name: &'static str, agent: PlayerAgent) -> Self{
        Self{
            name,
            agent,
            board_half: [2;16],
        }
    }
}

struct Game{
    direction: Direction,
    mode: Mode,
    turn: u64,
    player1: Player,
    player2: Player,
}

impl Game{
    fn new(direction: Direction, mode: Mode, player1: Player, player2: Player) -> Self{
        Self{
            direction,
            mode,
            turn: 1,
            player1,
            player2,
        }
    }

    fn run(&mut self) {
        while self.move_possible() && !self.game_over(){
            self.print_board();
            self.make_move();
            self.turn += 1;
        }

        // the current player could not make a move or its loosing condition was met
        // therefore we skip the player to display the winner
        self.turn += 1;
        print!("Congratulation {}. You won!", self.get_current_player().name);
    }

    #[inline(always)]
    fn get_current_and_opponent_player(&mut self) -> (&mut Player, &mut Player){
        match self.turn % 2 {
            1 => (&mut self.player1, &mut self.player2),
            0 => (&mut self.player2, &mut self.player1),
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn get_current_player(&self) -> &Player {
        match self.turn % 2 {
            1 => &self.player1,
            2 => &self.player2,
            _ => unreachable!(),
        }
    }

    fn make_move(&mut self) {
        let (player, opponent) = self.get_current_and_opponent_player();
        let mut player_board = player.board_half;
        let mut opponent_board = opponent.board_half;

        let mut index= 0;
        let mut valid_index = false;
        while valid_index == false {
            println!("{} enter bowl index: ", player.name);
            
            index = Self::read_index();
            
            if (0..16).contains(&index) && player_board[index] >= 2 {
                valid_index = true;
            }else{
                print!("Please enter a valid index (0-15). Bowl must contain at least 2 stones.");
            }
        }

        let mut hand: u8 = player_board[index];
        player_board[index] = 0;

        while hand > 0 {
            index = self.next_index(index);
            hand = hand - 1;
            player_board[index] = player_board[index] + 1;

            if hand == 0 && player_board[index] >= 2 {
                hand = player_board[index];
                player_board[index] = 0;

                // steal from opponent
                if index >= 8 && index <= 15 {
                    hand = hand + match self.mode {
                        Mode::EASY => {
                            let steal = opponent_board[index];
                            opponent_board[index] = 0;
                            steal
                        },
                        Mode::NORMAL =>{
                            let steal = opponent_board[index] + opponent_board[index-7];
                            opponent_board[index] = 0;
                            opponent_board[index-7] = 0;
                            steal
                        },
                    }
                }
            }
        }        
    }

    #[inline(always)]
    fn next_index(&self, index: usize) -> usize {
        // I should use a ring for each player
        match self.turn % 2 {
            1 => {
                match self.direction {
                    Direction::CW => if index+1 > 15 {0} else {index+1},
                    Direction::CCW => if index < 1 {15} else {index-1},
                }
            },
            0 => {
                match self.direction {
                    Direction::CW => if index < 1 {15} else {index-1},
                    Direction::CCW => if index+1 > 15 {0} else {index+1},
                }
            },
            _ => panic!("How did we get here?`")
        }
    }

    fn read_index() -> usize{
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
                },
            };
        }
        index.unwrap()
    }

    fn print_board(&self){
        println!("           {:2}Player 2{:2}",
        if self.turn % 2 == 0 {"->"} else {""},
        if self.turn % 2 == 0  {"<-"} else {""},
        );
        print!("|");
        for i in 0..8 {
            print!(" {:2} |", i);
        }
        println!("");
        println!("-----------------------------------------");
        print!("|");
        for i in 0..8 {
            print!(" {:2} |", self.player2.board_half[i]);
        }
        println!("");

        println!("-----------------------------------------");
        print!("|");
        for i in (8..16).rev() {
            print!(" {:2} |", i);
        }
        println!("");
        println!("-----------------------------------------");
        print!("|");
        for i in (8..16).rev() {
            print!(" {:2} |", self.player2.board_half[i]);
        }
        println!(" Stones: {}", self.player2.board_half.iter().sum::<u8>());
        println!("==================================================== Round: {}", self.turn);
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
        println!("");
        println!("-----------------------------------------");
        print!("|");
        for i in 0..8 {
            print!(" {:2} |", self.player1.board_half[i]);
        }
        println!("");
        println!("-----------------------------------------");
        print!("|");
        for i in 0..8 {
            print!(" {:2} |", i);
        }
        println!("");
        println!("           {:2}Player 1{:2}",
         if self.turn % 2 == 1  {"->"} else {""},
         if self.turn % 2 == 1  {"<-"} else {""},
        );
    }

    fn move_possible(&self) -> bool{
        for bowl in self.get_current_player().board_half.iter(){
            if bowl >= &2 {
                return true
            }
        }
        print!("{}: no possible moveds left :(", self.get_current_player().name);
        false
    }

    fn game_over(&self)->bool{
        let current_player_board = self.get_current_player().board_half;

        match self.mode {
            Mode::EASY => {
                for bowl in 8..15 {
                    if current_player_board[bowl] != 0 {
                        return false
                    }
                }
                true
            },
            Mode::NORMAL => {
                for bowl in 0..15 {
                    if current_player_board[bowl] != 0 {
                        return false
                    }
                }
                true 
            },
        }
    }
}

fn main(){
    let mut game = Game::new(Direction::CW, Mode::EASY,
         Player::new("Player 1", PlayerAgent::HUMAN),
         Player::new("Player 2", PlayerAgent::HUMAN));
    game.run();
}
