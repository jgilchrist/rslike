use tcod::{BackgroundFlag, Console, Key, KeyCode};
use tcod::{FONT_LAYOUT_ASCII_INROW, FONT_TYPE_GREYSCALE};

use engine::Game;

pub struct GUI {
    game: Game,
    width: int,
    height: int,
    pub console: Console,
}

impl GUI {
    pub fn new(game: Game) -> GUI {
        let (width, height) = (100, 50);

        Console::set_custom_font(Path::new("/home/jonny/code/rslike/assets/fonts/terminal12x12_gs_ro.png"), FONT_LAYOUT_ASCII_INROW | FONT_TYPE_GREYSCALE, 0, 0);
        let console = Console::init_root(width, height, "Libtcod Rust Tutorial", false);

        GUI {
            game: game,
            width: width,
            height: height,
            console: console,
        }
    }

    pub fn run(&mut self) {
        while !self.window_closed() {
            self.render();

            let keypress = Console::wait_for_keypress(true);

            if let Key::Special(KeyCode::Escape) = keypress.key {
                break;
            }
        }
    }

    pub fn render(&mut self) {
        self.clear();

        self.console.put_char(40, 25, '@', BackgroundFlag::Set);

        self.flush();
    }

    pub fn clear(&mut self) {
        self.console.clear();
    }

    pub fn flush(&mut self) {
        Console::flush();
    }

    pub fn window_closed(&self) -> bool {
        Console::window_closed()
    }
}
