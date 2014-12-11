use tcod;
use tcod::{BackgroundFlag, Console, Key, KeyCode};
use tcod::{KEY_PRESSED, FONT_LAYOUT_ASCII_INROW, FONT_TYPE_GREYSCALE};

use engine::Game;
use util::units::{Direction, XYPair};

#[deriving(PartialEq)]
pub enum State {
    Running,
    Exited
}

pub struct GUI {
    game: Game,
    width: int,
    height: int,
    pub console: Console,
    state: State,
}

impl GUI {
    pub fn new(game: Game) -> GUI {
        let (width, height) = (100, 50);

        tcod::system::set_fps(60);
        Console::set_custom_font(Path::new("/home/jonny/code/rslike/assets/fonts/terminal12x12_gs_ro.png"), FONT_LAYOUT_ASCII_INROW | FONT_TYPE_GREYSCALE, 0, 0);
        let console = Console::init_root(width, height, "Libtcod Rust Tutorial", false);

        GUI {
            game: game,
            width: width,
            height: height,
            console: console,
            state: State::Running,
        }
    }

    pub fn run(&mut self) {
        while !self.window_closed() && !self.exited() {
            self.render();
            self.handle_input();
        }
    }

    fn handle_input(&mut self) {
        let check_key = Console::check_for_keypress(KEY_PRESSED);

        if let Some(keypress) = check_key {
            match keypress.key {
                Key::Special(KeyCode::Up) => {
                    self.game.walk(Direction::Up);
                },
                Key::Special(KeyCode::Down) => {
                    self.game.walk(Direction::Down);
                },
                Key::Special(KeyCode::Left) => {
                    self.game.walk(Direction::Left);
                },
                Key::Special(KeyCode::Right) => {
                    self.game.walk(Direction::Right);
                },
                Key::Special(KeyCode::Escape) => {
                    self.state = State::Exited;
                },
                _ => {}
            }
        }

    }

    fn render(&mut self) {
        self.clear();

        let (px, py) = self.game.world.player.pos().as_xy();
        let repr = self.game.world.player.repr();

        self.console.put_char(px, py, repr, BackgroundFlag::Set);

        self.flush();
    }

    pub fn clear(&mut self) {
        self.console.clear();
    }

    pub fn flush(&mut self) {
        Console::flush();
    }

    pub fn exited(&self) -> bool {
        self.state == State::Exited
    }

    pub fn window_closed(&self) -> bool {
        Console::window_closed()
    }
}
