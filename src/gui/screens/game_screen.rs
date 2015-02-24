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
        console.put_plain(self.map_location + Point::new(-1, -1), chars::NW as char);
        console.put_plain(self.map_location + Point::new(63, -1), chars::NE as char);
        console.put_plain(self.map_location + Point::new(-1, 38), chars::SW as char);
        console.put_plain(self.map_location + Point::new(63, 38), chars::SE as char);

        for x in 0..63 {
            console.put_plain(self.map_location + Point::new(x, -1), chars::HLINE as char);
            console.put_plain(self.map_location + Point::new(x, 38), chars::HLINE as char);
        }

        for y in 0..38 {
            console.put_plain(self.map_location + Point::new(-1, y), chars::VLINE as char);
            console.put_plain(self.map_location + Point::new(63, y), chars::VLINE as char);
        }

        console.put_plain(self.map_location + Point::new(0, -1), 'M');
        console.put_plain(self.map_location + Point::new(1, -1), 'a');
        console.put_plain(self.map_location + Point::new(2, -1), 'p');

        console.put_plain(self.info_location + Point::new(-1, -1), chars::NW as char);
        console.put_plain(self.info_location + Point::new(13, -1), chars::NE as char);
        console.put_plain(self.info_location + Point::new(-1, 48), chars::SW as char);
        console.put_plain(self.info_location + Point::new(13, 48), chars::SE as char);

        for x in 0..13 {
            console.put_plain(self.info_location + Point::new(x, -1), chars::HLINE as char);
            console.put_plain(self.info_location + Point::new(x, 48), chars::HLINE as char);
        }

        for y in 0..48 {
            console.put_plain(self.info_location + Point::new(-1, y), chars::VLINE as char);
            console.put_plain(self.info_location + Point::new(13, y), chars::VLINE as char);
        }

        console.put_plain(self.info_location + Point::new(0, -1), 'I');
        console.put_plain(self.info_location + Point::new(1, -1), 'n');
        console.put_plain(self.info_location + Point::new(2, -1), 'f');
        console.put_plain(self.info_location + Point::new(3, -1), 'o');

        console.put_plain(self.message_location + Point::new(-1, -1), chars::NW as char);
        console.put_plain(self.message_location + Point::new(63, -1), chars::NE as char);
        console.put_plain(self.message_location + Point::new(-1,  8), chars::SW as char);
        console.put_plain(self.message_location + Point::new(63,  8), chars::SE as char);

        for x in 0..63 {
            console.put_plain(self.message_location + Point::new(x, -1), chars::HLINE as char);
            console.put_plain(self.message_location + Point::new(x,  8), chars::HLINE as char);
        }

        for y in 0..8 {
            console.put_plain(self.message_location + Point::new(-1, y), chars::VLINE as char);
            console.put_plain(self.message_location + Point::new(63, y), chars::VLINE as char);
        }

        console.put_plain(self.message_location + Point::new(0, -1), 'M');
        console.put_plain(self.message_location + Point::new(1, -1), 'e');
        console.put_plain(self.message_location + Point::new(2, -1), 's');
        console.put_plain(self.message_location + Point::new(3, -1), 's');
        console.put_plain(self.message_location + Point::new(4, -1), 'a');
        console.put_plain(self.message_location + Point::new(5, -1), 'g');
        console.put_plain(self.message_location + Point::new(6, -1), 'e');
        console.put_plain(self.message_location + Point::new(7, -1), 's');

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
