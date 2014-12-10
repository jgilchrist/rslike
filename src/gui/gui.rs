use tcod::{BackgroundFlag, Console, Key, KeyCode};
use tcod::{FONT_LAYOUT_ASCII_INROW, FONT_TYPE_GREYSCALE};

pub struct GUI {
    width: int,
    height: int,
    pub console: Console,
}

impl GUI {
    pub fn new(width: int, height: int) -> GUI {
        Console::set_custom_font(Path::new("/home/jonny/code/rslike/assets/fonts/terminal12x12_gs_ro.png"), FONT_LAYOUT_ASCII_INROW | FONT_TYPE_GREYSCALE, 0, 0);
        let console = Console::init_root(width, height, "Libtcod Rust Tutorial", false);

        GUI {
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
