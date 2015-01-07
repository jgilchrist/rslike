use engine::Game;
use gui::{Console, Colors, Key};
use util::units::{Direction, Point, Size};

#[derive(PartialEq)]
pub enum State {
    Running,
    Exited
}

pub struct GUI<'a> {
    pub console: &'a mut (Console + 'a),
    game: Game,
    state: State,
}

impl<'a> GUI<'a> {
    pub fn new<T>(game: Game, console: &'a mut T) -> GUI where T: Console {
        GUI {
            game: game,
            console: console,
            state: State::Running,
        }
    }

    pub fn size(&self) -> Size {
        self.console.size()
    }

    pub fn run(&mut self) {
        while !self.console.window_closed() && !self.exited() {
            self.render();
            self.handle_input();
        }
    }

    fn handle_input(&mut self) {
        if let Some(key) = self.console.check_for_keypress() {
            match key {
                Key::Up => {
                    self.game.walk(Direction::Up);
                },
                Key::Down => {
                    self.game.walk(Direction::Down);
                },
                Key::Left => {
                    self.game.walk(Direction::Left);
                },
                Key::Right => {
                    self.game.walk(Direction::Right);
                },
                Key::Escape => {
                    self.state = State::Exited;
                },
            }
        }

    }

    fn render(&mut self) {
        self.console.clear();

        self.render_map();

        let repr = self.game.world.player.repr;
        let pos = self.game.world.player.pos;

        self.console.put_plain(pos, repr);

        self.console.flush();
    }

    fn render_map(&mut self) {
        let map = &(self.game.world.map);

        for (y, line) in map.tiles.iter().enumerate() {
            for (x, cell) in line.iter().enumerate() {
                self.console.put(Point::new(x as int, y as int), ' ', Colors::white, cell.b_color());
            }
        }
    }

    pub fn exited(&self) -> bool {
        self.state == State::Exited
    }
}
