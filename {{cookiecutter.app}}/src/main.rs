
mod game;
use game::Game;

fn main() {
    
    let mut game = Game{..Default::default()};

    game.run_game();
    
}
