use engine::Game;
use gui::screens::{self, Screen, ScreenChange};
use gui::{Console, Colors, Key};
use util::units::{Direction, Point};

#[allow(missing_copy_implementations)]
pub struct GameScreen {
    map_location: Point,
}

impl GameScreen {
    pub fn new() -> Box<Screen> {
        Box::new(
            GameScreen {
                map_location: Point::new(16, 1)
            }
        )
    }
}

impl Screen for GameScreen {

    #[allow(unused)]
    fn input(&mut self, game: &mut Game, console: &mut Console) -> Option<ScreenChange> {
        if let Some(key) = console.check_for_keypress() {
            match key {
                Key::Up => {
                    game.world.walk(Direction::Up);
                },
                Key::Down => {
                    game.world.walk(Direction::Down);
                },
                Key::Left => {
                    game.world.walk(Direction::Left);
                },
                Key::Right => {
                    game.world.walk(Direction::Right);
                },
                Key::Escape => {
                    return Some(ScreenChange::AddScreen(screens::PauseScreen::new()));
                },
                _ => {}
            }
        }

        None
    }

    #[allow(unused)]
    fn update(&mut self, game: &mut Game, console: &mut Console) -> Option<ScreenChange> {
        return None;
    }

    #[allow(unused)]
    fn render(&mut self, game: &mut Game, console: &mut Console) {
        self.render_map(game, console);

        let repr = game.world.player.repr;
        let pos = game.world.player.pos;

        console.put_plain(self.map_location + pos, repr);

    }
}

impl GameScreen {

    fn render_map(&self, game: &mut Game, console: &mut Console) {
        let map = &(game.world.map);

        for (y, line) in map.tiles.iter().enumerate() {
            for (x, cell) in line.iter().enumerate() {
                console.put(self.map_location + Point::new(x as i32, y as i32), ' ', Colors::white, cell.b_color());
            }
        }
    }

}
