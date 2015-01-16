use gui::{Console, Color, Key};
use util::units::{AsTuple, Point, Size};

use tcod;

pub struct TCODConsole {
    console: tcod::Console,
    size: Size,
}

impl TCODConsole {

    pub fn new(size: Size) -> TCODConsole {
        tcod::system::set_fps(60);
        tcod::Console::set_custom_font(Path::new("/home/jonny/code/rslike/assets/fonts/terminal12x12_gs_ro.png"), tcod::FONT_LAYOUT_ASCII_INROW | tcod::FONT_TYPE_GREYSCALE, 0, 0);

        let (width, height) = size.as_tuple();
        let console = tcod::Console::init_root(width, height, "rslike", false);

        TCODConsole {
            console: console,
            size: size,
        }
    }

}

impl Console for TCODConsole {
    fn put_plain(&mut self, pos: Point, c: char) {
        self.console.put_char(pos.x, pos.y, c, tcod::BackgroundFlag::None);
    }

    fn put(&mut self, pos: Point, c: char, f_color: Color, b_color: Color) {
        self.console.put_char_ex(pos.x, pos.y, c, f_color, b_color);
    }

    fn clear(&mut self) {
        self.console.clear();
    }

    fn flush(&mut self) {
        tcod::Console::flush();
    }

    fn check_for_keypress(&mut self) -> Option<Key> {
        let check_key = tcod::Console::check_for_keypress(tcod::KEY_PRESSED);

        match check_key {
            Some(keypress) => {
                match keypress.key {
                    tcod::Key::Special(tcod::KeyCode::Up) => Some(Key::Up),
                    tcod::Key::Special(tcod::KeyCode::Down) => Some(Key::Down),
                    tcod::Key::Special(tcod::KeyCode::Left) => Some(Key::Left),
                    tcod::Key::Special(tcod::KeyCode::Right) => Some(Key::Right),
                    tcod::Key::Special(tcod::KeyCode::Escape) => Some(Key::Escape),
                    _ => None
                }
            }
            _ => None
        }
    }

    fn size(&self) -> Size {
        self.size
    }

    fn window_closed(&self) -> bool {
        tcod::Console::window_closed()
    }
}
