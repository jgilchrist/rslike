extern crate tcod;
extern crate rslike;

use rslike::gui::{GUI, TCODConsole};
use rslike::engine::Game;
use rslike::util::units::Size;

fn main() {
    let game = Game::new();
    let mut tcod_console = TCODConsole::new(Size::new(100, 50));
    let mut gui = GUI::new(game, &mut tcod_console);

    gui.run();
}
