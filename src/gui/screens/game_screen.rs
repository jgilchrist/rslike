use engine::Game;
use gui::screens::{self, Screen, ScreenChange};
use gui::{Console, Colors, Key};
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
        let CHAR_VLINE = 179 as char;
        let CHAR_HLINE = 196 as char;
        let CHAR_CORNER_NW = 218 as char;
        let CHAR_CORNER_NE = 191 as char;
        let CHAR_CORNER_SW = 192 as char;
        let CHAR_CORNER_SE = 217 as char;

        console.put_plain(self.map_location + Point::new(-1, -1), CHAR_CORNER_NW);
        console.put_plain(self.map_location + Point::new(63, -1), CHAR_CORNER_NE);
        console.put_plain(self.map_location + Point::new(-1, 38), CHAR_CORNER_SW);
        console.put_plain(self.map_location + Point::new(63, 38), CHAR_CORNER_SE);

        for x in 0..63 {
            console.put_plain(self.map_location + Point::new(x, -1), CHAR_HLINE);
            console.put_plain(self.map_location + Point::new(x, 38), CHAR_HLINE);
        }

        for y in 0..38 {
            console.put_plain(self.map_location + Point::new(-1, y), CHAR_VLINE);
            console.put_plain(self.map_location + Point::new(63, y), CHAR_VLINE);
        }

        console.put_plain(self.map_location + Point::new(0, -1), 'M');
        console.put_plain(self.map_location + Point::new(1, -1), 'a');
        console.put_plain(self.map_location + Point::new(2, -1), 'p');

        console.put_plain(self.info_location + Point::new(-1, -1), CHAR_CORNER_NW);
        console.put_plain(self.info_location + Point::new(13, -1), CHAR_CORNER_NE);
        console.put_plain(self.info_location + Point::new(-1, 48), CHAR_CORNER_SW);
        console.put_plain(self.info_location + Point::new(13, 48), CHAR_CORNER_SE);

        for x in 0..13 {
            console.put_plain(self.info_location + Point::new(x, -1), CHAR_HLINE);
            console.put_plain(self.info_location + Point::new(x, 48), CHAR_HLINE);
        }

        for y in 0..48 {
            console.put_plain(self.info_location + Point::new(-1, y), CHAR_VLINE);
            console.put_plain(self.info_location + Point::new(13, y), CHAR_VLINE);
        }

        console.put_plain(self.info_location + Point::new(0, -1), 'I');
        console.put_plain(self.info_location + Point::new(1, -1), 'n');
        console.put_plain(self.info_location + Point::new(2, -1), 'f');
        console.put_plain(self.info_location + Point::new(3, -1), 'o');

        console.put_plain(self.message_location + Point::new(-1, -1), CHAR_CORNER_NW);
        console.put_plain(self.message_location + Point::new(63, -1), CHAR_CORNER_NE);
        console.put_plain(self.message_location + Point::new(-1,  8), CHAR_CORNER_SW);
        console.put_plain(self.message_location + Point::new(63,  8), CHAR_CORNER_SE);

        for x in 0..63 {
            console.put_plain(self.message_location + Point::new(x, -1), CHAR_HLINE);
            console.put_plain(self.message_location + Point::new(x,  8), CHAR_HLINE);
        }

        for y in 0..8 {
            console.put_plain(self.message_location + Point::new(-1, y), CHAR_VLINE);
            console.put_plain(self.message_location + Point::new(63, y), CHAR_VLINE);
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
