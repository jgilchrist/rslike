extern crate tcod;
extern crate rslike;

use rslike::gui::GUI;
use rslike::engine::Game;

fn main() {
    let game = Game::new();
    let mut gui = GUI::new(game);

    gui.run();
}
