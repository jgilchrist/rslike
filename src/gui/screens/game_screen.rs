use engine::Game;
use gui::screens::{self, Screen, ScreenChange};
use gui::{Console, Colors, Key};
use gui::chars;
use util::units::{Direction, Point};

#[allow(missing_copy_implementations)]
pub struct GameScreen {
    map_location: Point,
    info_location: Point,
    message_location: Point,
}

impl GameScreen {
    pub fn new() -> Box<Screen> {
        Box::new(
            GameScreen {
                map_location: Point::new(16, 1),
                info_location: Point::new(1, 1),
                message_location: Point::new(16, 41),
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

        self.draw_borders(game, console);
    }
}

impl GameScreen {

    #[allow(unused)]
    fn draw_borders(&self, game: &mut Game, console: &mut Console) {
        self.draw_box(console, self.map_location + Point::new(-1, -1), 64, 39);
        console.print_plain(self.map_location + Point::new(0, -1), "Map");

        self.draw_box(console, self.info_location + Point::new(-1, -1), 14, 49);
        console.print_plain(self.info_location + Point::new(0, -1), "Info");

        self.draw_box(console, self.message_location + Point::new(-1, -1), 64, 9);
        console.print_plain(self.message_location + Point::new(0, -1), "Messages");
    }

    fn draw_box(&self, console: &mut Console, loc: Point, width: i32, height: i32) {
        console.put_plain(loc + Point::new(0, 0), chars::NW as char);
        console.put_plain(loc + Point::new(width, 0), chars::NE as char);
        console.put_plain(loc + Point::new(0, height), chars::SW as char);
        console.put_plain(loc + Point::new(width, height), chars::SE as char);

        for x in 1..width {
            console.put_plain(loc + Point::new(x, 0), chars::HLINE as char);
            console.put_plain(loc + Point::new(x, height), chars::HLINE as char);
        }

        for y in 1..height {
            console.put_plain(loc + Point::new(0, y), chars::VLINE as char);
            console.put_plain(loc + Point::new(width, y), chars::VLINE as char);
        }
    }

    #[allow(unused)]
    fn render_map(&self, game: &mut Game, console: &mut Console) {
        let map = &(game.world.map);

        for (y, line) in map.tiles.iter().enumerate() {
            for (x, cell) in line.iter().enumerate() {
                console.put(self.map_location + Point::new(x as i32, y as i32), ' ', Colors::white, cell.b_color());
            }
        }
    }

}
