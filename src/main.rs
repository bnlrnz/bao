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
            direction: direction,
            mode: mode,
            turn: 1
        }
    }

    fn run(&mut self) {
        while self.move_possible(){
            self.print_board();
            self.make_move();
            self.turn += 1;
        }
    }

    fn get_current_player_and_oppenent(&mut self) -> (&mut [u8], &mut [u8]){
        if self.turn % 2 == 1 {
            return (&mut self.board.player1, &mut self.board.player2);
        }else{
            return (&mut self.board.player2, &mut self.board.player1);
        }
    }

    fn make_move(&mut self) {
        let turn = self.turn;
        let direction = self.direction;
        let mode = self.mode;

        let mut index = self.read_index();
        
        let (player, opponent) = self.get_current_player_and_oppenent();

        let mut hand: u8 = player[index];
        player[index] = 0;

        while hand > 0 {
            index = Self::next_index(index, turn, direction);
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

    fn next_index(index: usize, turn: u64, direction: Direction) -> usize {
        // I should use a ring for each player
        if turn % 2 == 1 {
            match direction {
                Direction::CW => if index+1 > 15 {0} else {index+1},
                Direction::CCW => if index-1 < 1 {15} else {index-1},
            }
        }else{
            match direction {
                Direction::CW => if index-1 < 1 {15} else {index-1},
                Direction::CCW => if index+1 > 15 {0} else {index+1},
            }
        }
    }

    fn read_index(&self) -> usize{
        let mut index: Option<usize> = None;
        while index == None {
            println!("{} enter bowl index: ", if self.turn % 2 == 1 {"Player 1"} else {"Player 2"});
            let mut input_text = String::new();
            io::stdin()
                .read_line(&mut input_text)
                .expect("failed to read from stdin");
            
            let trimmed = input_text.trim();
            index = match trimmed.parse::<usize>() {
                Ok(i) => Some(i),
                Err(..) => None,
            };

            if index == None{
                println!("Please enter a valid number!");
                continue;
            }

            if index.unwrap() > 15 {
                println!("Please enter a valid number (0-15)!");
                index = None;
                continue;
            }

            if self.turn % 2 == 1{
                if self.board.player1[index.unwrap()] < 2{
                    println!("Player1: Choose a bowl with at least 2 stones!");
                    index = None
                }
            }else{
                if self.board.player2[index.unwrap()] < 2{
                    println!("Player2: Choose a bowl with at least 2 stones!");
                    index = None
                }
            }
        }

        return index.unwrap();
    }

    fn print_board(&self){
        println!("           {:2}Player 2{:2}",
         if self.turn % 2 == 0 {"->"} else {""},
         if self.turn % 2 == 0 {"<-"} else {""},
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
        println!("=========================================");
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
         if self.turn % 2 == 1 {"->"} else {""},
         if self.turn % 2 == 1 {"<-"} else {""},
        );
    }

    fn move_possible(&self) -> bool{
        if self.turn % 2 == 1 {
            for bowl in self.board.player1.iter() {
                if bowl >= &2 {
                    return true
                }
            }
            println!("Player 1: no possible move left :(");
            false
        }else{
            for bowl in self.board.player2.iter() {
                if bowl >= &2 {
                    return true
                }
            }
            println!("Player 2: no possible move left :(");
            false
        }
    }
}

fn main(){
    Game::new(Direction::CW, Mode::NORMAL).run();
}
