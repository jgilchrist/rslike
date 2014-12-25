extern crate tcod;
extern crate rslike;

use rslike::gui::{GUI, TCODConsole};
use rslike::engine::Game;
use rslike::util::units::Size;

fn main() {
    let game = Game::new();
    let mut tcod_console = TCODConsole::new(Size::new(114, 71));
    let mut gui = GUI::new(game, &mut tcod_console);

    gui.run();
}
