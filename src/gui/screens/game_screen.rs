use engine::Game;
use gui::{Console, Colors, Key, Screen, ScreenChange};
use util::units::{Direction, Point};

#[allow(missing_copy_implementations)]
pub struct GameScreen;

impl GameScreen {
    pub fn new() -> Box<Screen + 'static> {
        Box::new(GameScreen)
    }
}

impl Screen for GameScreen {

    fn input(&self, game: &mut Game, console: &mut Console) -> Option<ScreenChange> {
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
                    return Some(ScreenChange::ExitGame);
                },
            }
        }

        None
    }

    fn update(&self, game: &mut Game, console: &mut Console) -> Option<ScreenChange> {
        return None;
    }

    fn render(&self, game: &mut Game, console: &mut Console) {
        self.render_map(game, console);

        let repr = game.world.player.repr;
        let pos = game.world.player.pos;

        console.put_plain(pos, repr);

    }
}

impl GameScreen {

    fn render_map(&self, game: &mut Game, console: &mut Console) {
        let map = &(game.world.map);

        for (y, line) in map.tiles.iter().enumerate() {
            for (x, cell) in line.iter().enumerate() {
                console.put(Point::new(x as i32, y as i32), ' ', Colors::white, cell.b_color());
            }
        }
    }

}
