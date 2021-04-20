mod bao;
use bao::*;

fn main() {
    let mut game = Game::new(
        Direction::CW,
        Mode::EASY,
        Player::new("Player 1", PlayerAgent::HUMAN),
        Player::new("Player 2", PlayerAgent::HUMAN),
    );

    game.run();
}
