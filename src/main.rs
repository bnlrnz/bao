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
enum Player{
    PLAYER1,
    PLAYER2,
}

struct Board{
    player1: [u8; 16],
    player2: [u8; 16],
}

impl Board{
    fn new() -> Self {
        Self {
            player1: [2; 16],
            player2: [2; 16],
        }
    }
}

struct Game{
    board: Board,
    direction: Direction,
    mode: Mode,
    turn: u64,
}

impl Game{
    fn new(direction: Direction, mode: Mode) -> Self{
        Self{
            board: Board::new(),
            direction,
            mode,
            turn: 1
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
        match self.get_current_player() {
            Player::PLAYER1 => print!("Player 1 won!"),
            Player::PLAYER2 => print!("Player 2 won!"),
        }
    }

    #[inline(always)]
    fn get_current_player(&self) -> Player {
        match self.turn % 2 {
            1 => Player::PLAYER1,
            0 => Player::PLAYER2,
            _ => panic!("dafuq could dis happen?"),
        }
    }

    #[inline(always)]
    fn get_current_player_and_oppenent_array(&mut self) -> (&mut [u8], &mut [u8]){
        match self.get_current_player(){
            Player::PLAYER1 => return (&mut self.board.player1, &mut self.board.player2),
            Player::PLAYER2 => return (&mut self.board.player2, &mut self.board.player1),
        }
    }

    fn make_move(&mut self) {
        let direction = self.direction;
        let mode = self.mode;
        let current_player = self.get_current_player();
        let (player, opponent) = self.get_current_player_and_oppenent_array();

        let mut index= 0;
        let mut valid_index = false;
        while valid_index == false {
            println!("{} enter bowl index: ", if current_player == Player::PLAYER1 {"Player 1"} else {"Player 2"});
            
            index = Self::read_index();
            
            if (0..16).contains(&index) && player[index] >= 2 {
                valid_index = true;
            }else{
                print!("Please enter a valid index (0-15). Bowl must contain at least 2 stones.");
            }
        }

        let mut hand: u8 = player[index];
        player[index] = 0;

        while hand > 0 {
            index = Self::next_index(index, current_player, direction);
            hand = hand - 1;
            player[index] = player[index] + 1;

            if hand == 0 && player[index] >= 2 {
                hand = player[index];
                player[index] = 0;

                // steal from opponent
                if index >= 8 && index <= 15 {
                    hand = hand + match mode {
                        Mode::EASY => {
                            let steal = opponent[index];
                            opponent[index] = 0;
                            steal
                        },
                        Mode::NORMAL =>{
                            let steal = opponent[index] + opponent[index-7];
                            opponent[index] = 0;
                            opponent[index-7] = 0;
                            steal
                        },
                    }
                }
            }
        }        
    }

    #[inline(always)]
    fn next_index(index: usize, player: Player, direction: Direction) -> usize {
        // I should use a ring for each player
        match player {
            Player::PLAYER1 => {
                match direction {
                    Direction::CW => if index+1 > 15 {0} else {index+1},
                    Direction::CCW => if index-1 < 1 {15} else {index-1},
                }
            },
            Player::PLAYER2 => {
                match direction {
                    Direction::CW => if index-1 < 1 {15} else {index-1},
                    Direction::CCW => if index+1 > 15 {0} else {index+1},
                }
            },
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
        let player = self.get_current_player();

        println!("           {:2}Player 2{:2}",
        if player == Player::PLAYER2 {"->"} else {""},
        if player == Player::PLAYER2 {"<-"} else {""},
        );
        print!("|");
        for i in 0..8 {
            print!(" {:2} |", i);
        }
        println!("");
        println!("-----------------------------------------");
        print!("|");
        for i in 0..8 {
            print!(" {:2} |", self.board.player2[i]);
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
            print!(" {:2} |", self.board.player2[i]);
        }
        println!(" Stones: {}", self.board.player2.iter().sum::<u8>());
        println!("==================================================== Round: {}", self.turn);
        print!("|");
        for i in (8..16).rev() {
            print!(" {:2} |", self.board.player1[i]);
        }
        println!(" Stones: {}", self.board.player1.iter().sum::<u8>());
        println!("-----------------------------------------");
        print!("|");
        for i in (8..16).rev() {
            print!(" {:2} |", i);
        }
        println!("");
        println!("-----------------------------------------");
        print!("|");
        for i in 0..8 {
            print!(" {:2} |", self.board.player1[i]);
        }
        println!("");
        println!("-----------------------------------------");
        print!("|");
        for i in 0..8 {
            print!(" {:2} |", i);
        }
        println!("");
        println!("           {:2}Player 1{:2}",
         if player == Player::PLAYER1 {"->"} else {""},
         if player == Player::PLAYER1 {"<-"} else {""},
        );
    }

    fn move_possible(&self) -> bool{
        match self.get_current_player() {
            Player::PLAYER1 => {
                for bowl in self.board.player1.iter() {
                    if bowl >= &2 {
                        return true
                    }
                }
                println!("Player 1: no possible move left :(");
                false
            },
            Player::PLAYER2 => {
                for bowl in self.board.player2.iter() {
                    if bowl >= &2 {
                        return true
                    }
                }
                println!("Player 2: no possible move left :(");
                false
            },
        }
    }

    fn game_over(&self)->bool{
        let current_player_array =
            match self.get_current_player() {
                Player::PLAYER1 => self.board.player1,
                Player::PLAYER2 => self.board.player2,
            };

        match self.mode {
            Mode::EASY => {
                for bowl in 8..15 {
                    if current_player_array[bowl] != 0 {
                        return false
                    }
                }
                true
            },
            Mode::NORMAL => {
                for bowl in 0..15 {
                    if current_player_array[bowl] != 0 {
                        return false
                    }
                }
                true 
            },
        }
    }
}

fn main(){
    Game::new(Direction::CW, Mode::EASY).run();
}
