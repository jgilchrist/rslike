extern crate tcod;
extern crate rslike;

use rslike::gui::GUI;

fn main() {
    println!("Hello, world!");

    let mut gui = GUI::new(100, 60);

    gui.run();
}
