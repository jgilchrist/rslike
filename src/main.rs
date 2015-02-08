extern crate tcod;
extern crate rslike;

use rslike::gui::{Console, GUI};
use rslike::engine::Game;
use rslike::util::units::Size;

fn main() {
    let game = Game::new();
    let console = Console::new(Size::new(80, 50));
    let mut gui = GUI::new(game, console);

    gui.run();
}
