mod human_agent;
pub use human_agent::HumanAgent;

mod random_agent;
pub use random_agent::RandomAgent;

mod training_rustneat_agent;
pub use training_rustneat_agent::RustNeatAgent;

mod training_radiate_agent;
pub use training_radiate_agent::RadiateAgent;

mod maximize_agent;
pub use maximize_agent::MaximizeAgent;

#[derive(PartialEq, Eq)]
enum Turn {
    Player1,
    Player2,
}

use Turn::*;

enum MoveResult {
    None(u8),
    Lost(u8),
    Won(u8),
}

#[allow(unused)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    CW,
    CCW,
}

impl Direction {
    #[inline(always)]
    fn input_enc(&self) -> f32 {
        match self {
            Direction::CW => 0.0,
            Direction::CCW => 1.0,
        }
    }
}

impl Direction {
    #[inline(always)]
    fn next_index(&self, index: usize) -> usize {
        match self {
            Direction::CW => {
                if index == 0 {
                    15
                } else {
                    index - 1
                }
            }

            Direction::CCW => {
                if index == 15 {
                    0
                } else {
                    index + 1
                }
            }
        }
    }
}

#[allow(unused)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Mode {
    Normal, // all stones required
    Easy,   // just the inner row must be empty to win
}

#[derive(Clone, Debug)]
pub struct Player {
    name: String,
    tag: usize,
    board_half: [u8; 16],
}

impl Player {
    pub fn new(name: &str, tag: usize) -> Self {
        Self {
            name: String::from(name),
            tag,
            board_half: [2; 16],
        }
    }

    #[inline(always)]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[inline(always)]
    pub fn tag(&self) -> usize {
        self.tag
    }

    #[inline(always)]
    fn is_valid_index(&self, index: usize) -> bool {
        self.board_half[index] > 1
    }

    #[inline(always)]
    fn has_lost(&self, mode: Mode) -> bool {
        // If there is at most one stone per bowl: lost
        if self.board_half.iter().all(|&bowl| bowl < 2) {
            return true;
        }

        // If this is easy mode and the inner row is empty: lost
        if (mode == Mode::Easy) && self.board_half.iter().skip(8).all(|&bowl| bowl == 0) {
            return true;
        }

        // Otherwise: not lost
        false
    }
}

pub trait Agent {
    fn pick_index(&mut self, game: &Game) -> usize;
}

pub struct GameResult {
    pub winner: Player,
    pub loser: Player,
    pub turn_count: usize,
}

pub struct Game {
    direction: Direction,
    mode: Mode,
    turn_count: usize,
    player1: Player,
    player2: Player,
}

impl Game {
    pub fn new(direction: Direction, mode: Mode, player1: Player, player2: Player) -> Self {
        Self {
            direction,
            mode,
            turn_count: 1,
            player1,
            player2,
        }
    }

    pub fn play<A1: Agent, A2: Agent>(mut self, agent1: &mut A1, agent2: &mut A2) -> GameResult {
        let (winner, loser) = loop {
            let move_result = if self.turn() == Player1 {
                self.make_move(agent1)
            } else {
                self.make_move(agent2)
            };

            match (move_result, self.turn()) {
                (MoveResult::Won(_), Player1) | (MoveResult::Lost(_), Player2) => {
                    break (self.player1, self.player2)
                }
                (MoveResult::Lost(_), Player1) | (MoveResult::Won(_), Player2) => {
                    break (self.player2, self.player1)
                }
                _ => self.turn_count += 1,
            }
        };

        GameResult {
            winner,
            loser,
            turn_count: self.turn_count,
        }
    }

    #[inline(always)]
    fn turn(&self) -> Turn {
        if (self.turn_count % 2) == 1 {
            Player1
        } else {
            Player2
        }
    }

    fn make_move<A: Agent>(&mut self, agent: &mut A) -> MoveResult {
        let mut index = agent.pick_index(self);

        let (mut player, mut opponent) = if self.turn() == Player1 {
            (&mut self.player1, &mut self.player2)
        } else {
            (&mut self.player2, &mut self.player1)
        };

        debug_assert!(
            (index < 16) && player.is_valid_index(index),
            "Invalid index"
        );

        let mut total_steal = 0;

        let mut hand = player.board_half[index];
        player.board_half[index] = 0;

        while hand > 0 {
            index = self.direction.next_index(index);
            hand -= 1;
            player.board_half[index] += 1;

            if hand == 0 && player.board_half[index] >= 2 {
                hand = player.board_half[index];
                player.board_half[index] = 0;

                // steal from opponent
                if (8..=15).contains(&index) {
                    let opponent_index = (15 - index) + 8;

                    hand += match self.mode {
                        Mode::Easy => {
                            let steal = opponent.board_half[opponent_index];
                            total_steal += steal;
                            opponent.board_half[opponent_index] = 0;
                            steal
                        }
                        Mode::Normal => {
                            let opponent_2nd_index = 15 - opponent_index;
                            let steal = opponent.board_half[opponent_index]
                                + opponent.board_half[opponent_2nd_index];
                            total_steal += steal;
                            opponent.board_half[opponent_index] = 0;
                            opponent.board_half[opponent_2nd_index] = 0;
                            steal
                        }
                    };

                    // check win condition after steal!
                    if opponent.has_lost(self.mode) {
                        return MoveResult::Won(total_steal);
                    }
                }
            }
        }

        // check lose condition after move!
        if player.has_lost(self.mode) {
            MoveResult::Lost(total_steal)
        } else {
            MoveResult::None(total_steal)
        }
    }

    fn steal_dry_run(
        mut index: usize,
        direction: Direction,
        mode: Mode,
        mut player: &mut Player,
        mut opponent: &mut Player,
    ) -> MoveResult {
        let mut total_steal = 0;

        let mut hand = player.board_half[index];
        player.board_half[index] = 0;

        while hand > 0 {
            index = direction.next_index(index);
            hand -= 1;
            player.board_half[index] += 1;

            if hand == 0 && player.board_half[index] >= 2 {
                hand = player.board_half[index];
                player.board_half[index] = 0;

                // steal from opponent
                if (8..=15).contains(&index) {
                    let opponent_index = (15 - index) + 8;

                    hand += match mode {
                        Mode::Easy => {
                            let steal = opponent.board_half[opponent_index];
                            total_steal += steal;
                            opponent.board_half[opponent_index] = 0;
                            steal
                        }
                        Mode::Normal => {
                            let opponent_2nd_index = 15 - opponent_index;
                            let steal = opponent.board_half[opponent_index]
                                + opponent.board_half[opponent_2nd_index];
                            total_steal += steal;
                            opponent.board_half[opponent_index] = 0;
                            opponent.board_half[opponent_2nd_index] = 0;
                            steal
                        }
                    };

                    // check win condition after steal!
                    if opponent.has_lost(mode) {
                        // always favour the winning move over the max steal move
                        return MoveResult::None(99);
                    }
                }
            }
        }
        MoveResult::None(total_steal)
    }
}
