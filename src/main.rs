mod bao;
use bao::*;

fn main() {
    let mut game = Game::new(
        Direction::CW,
        Mode::Easy,
        Player::new("Player 1", PlayerAgent::Human),
        Player::new("Player 2", PlayerAgent::AiRandom),
    );

    game.run();
}
