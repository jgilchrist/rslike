use gui::{Color, Colors};
use util::units::{AsTuple, Point, Size};

use tcod;
use tcod::Console as TCODConsole;
use tcod::input::Key as TKey;
use tcod::input::KeyCode as TKeyCode;

use std::path::Path;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Key {
    Up,
    Down,
    Left,
    Right,
    Escape,
    Enter,
    Char(char)
}

pub struct Console {
    console: tcod::RootConsole,
    size: Size,
}

impl Console {
    pub fn new(size: Size) -> Console {
        tcod::system::set_fps(60);

        let (width, height) = size.as_tuple();
        let console = tcod::RootConsole::initializer()
                        .size(width, height)
                        .title("rslike")
                        .font(Path::new("assets/fonts/terminal12x12_gs_ro.png"), tcod::FontLayout::AsciiInRow)
                        .font_type(tcod::FontType::Greyscale)
                        .init();

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
        self.console.set_default_background(Colors::BLACK);
        self.console.set_default_foreground(Colors::WHITE);
    }

    pub fn print_align(&mut self, pos: Point, text: &str, alignment: tcod::TextAlignment) {
        self.console.print_ex(pos.x, pos.y, tcod::BackgroundFlag::None, alignment, text);
    }

    pub fn clear(&mut self) {
        self.console.clear();
    }

    pub fn flush(&mut self) {
        self.console.flush();
    }

    pub fn check_for_keypress(&mut self) -> Option<Key> {
        let check_keypress = self.console.check_for_keypress(tcod::input::KEY_PRESSED);

        if let Some(keypress) = check_keypress {
            match keypress {
                TKey { code: TKeyCode::Up, .. } => Some(Key::Up),
                TKey { code: TKeyCode::Down, .. } => Some(Key::Down),
                TKey { code: TKeyCode::Left, .. } => Some(Key::Left),
                TKey { code: TKeyCode::Right, .. } => Some(Key::Right),
                TKey { code: TKeyCode::Escape, .. } => Some(Key::Escape),
                TKey { code: TKeyCode::Enter, .. } => Some(Key::Enter),
                TKey { code: TKeyCode::Char, .. } => Some(Key::Char(keypress.printable)),
                _ => None
            }
        } else {
            None
        }
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn window_closed(&self) -> bool {
        self.console.window_closed()
    }
}
