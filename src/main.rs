mod game;

use game::*;

fn main() {
    let mut game = Game::new();

    loop {
        game.print();
        game.take_turn()
    }
}
