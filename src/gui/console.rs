use gui::{Color, Colors};
use util::units::{AsTuple, Point, Size};

// TODO: Remove this import when std::prelude contains the new path
use std::path::Path;

use tcod;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Key {
    Up,
    Down,
    Left,
    Right,
    Escape,
    Enter,
}

pub struct Console {
    console: tcod::Console,
    size: Size,
}

impl Console {

    pub fn new(size: Size) -> Console {
        tcod::system::set_fps(60);
        tcod::Console::set_custom_font(Path::new("assets/fonts/terminal12x12_gs_ro.png"), tcod::FONT_LAYOUT_ASCII_INROW | tcod::FONT_TYPE_GREYSCALE, 0, 0);

        let (width, height) = size.as_tuple();
        let console = tcod::Console::init_root(width, height, "rslike", false);

        Console {
            console: console,
            size: size,
        }
    }

}

impl Console {

    pub fn put_plain(&mut self, pos: Point, c: char) {
        self.console.put_char(pos.x, pos.y, c, tcod::BackgroundFlag::None);
    }

    pub fn put(&mut self, pos: Point, c: char, f_color: Color, b_color: Color) {
        self.console.put_char_ex(pos.x, pos.y, c, f_color, b_color);
    }

    pub fn print_plain(&mut self, pos: Point, text: &str) {
        self.console.print_ex(pos.x, pos.y, tcod::BackgroundFlag::None, tcod::TextAlignment::Left, text);
    }

    pub fn print(&mut self, pos: Point, text: &str, f_color: Color, b_color: Color) {
        self.console.set_default_background(b_color);
        self.console.set_default_foreground(f_color);
        self.console.print_ex(pos.x, pos.y, tcod::BackgroundFlag::None, tcod::TextAlignment::Left, text);
        self.console.set_default_background(Colors::black);
        self.console.set_default_foreground(Colors::white);
    }

    pub fn print_align(&mut self, pos: Point, text: &str, alignment: tcod::TextAlignment) {
        self.console.print_ex(pos.x, pos.y, tcod::BackgroundFlag::None, alignment, text);
    }

    pub fn clear(&mut self) {
        self.console.clear();
    }

    pub fn flush(&mut self) {
        tcod::Console::flush();
    }

    pub fn check_for_keypress(&mut self) -> Option<Key> {
        let check_key = tcod::Console::check_for_keypress(tcod::KEY_PRESSED);

        match check_key {
            Some(keypress) => {
                match keypress.key {
                    tcod::Key::Special(tcod::KeyCode::Up) => Some(Key::Up),
                    tcod::Key::Special(tcod::KeyCode::Down) => Some(Key::Down),
                    tcod::Key::Special(tcod::KeyCode::Left) => Some(Key::Left),
                    tcod::Key::Special(tcod::KeyCode::Right) => Some(Key::Right),
                    tcod::Key::Special(tcod::KeyCode::Escape) => Some(Key::Escape),
                    tcod::Key::Special(tcod::KeyCode::Enter) => Some(Key::Enter),
                    _ => None
                }
            }
            _ => None
        }
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn window_closed(&self) -> bool {
        tcod::Console::window_closed()
    }

}
