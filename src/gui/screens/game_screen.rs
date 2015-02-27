use engine::Game;
use gui::screens::{self, Screen, ScreenChange};
use gui::{Console, Colors, Key};
use gui::chars;
use util::Rectangle;
use util::units::{Direction, Point, Size};

#[allow(missing_copy_implementations)]
pub struct GameScreen {
    map_frame: Rectangle,
    map_view: Point,
    info_frame: Rectangle,
    message_frame: Rectangle,
}

impl GameScreen {
    pub fn new() -> Box<Screen> {
        Box::new(
            GameScreen {
                map_frame: Rectangle::new(Point::new(16, 1), Size::new(63, 38)),
                map_view: Point::new(0, 0),
                info_frame: Rectangle::new(Point::new(1, 1), Size::new(13, 48)),
                message_frame: Rectangle::new(Point::new(16, 41), Size::new(63, 8)),
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
        None
    }

    #[allow(unused)]
    fn render(&mut self, game: &mut Game, console: &mut Console) {
        self.draw_borders(game, console);
        self.draw_map(game, console);
        self.draw_player(game, console);
    }
}

impl GameScreen {

    #[allow(unused)]
    fn draw_borders(&self, game: &mut Game, console: &mut Console) {
        self.draw_box(console, self.map_frame.move_dir(Point::new(-1, -1)).resize(Size::new(1, 1)));
        console.print_plain(self.map_frame.move_dir(Point::new(0, -1)).location(), "Map");

        self.draw_box(console, self.info_frame.move_dir(Point::new(-1, -1)).resize(Size::new(1, 1)));
        console.print_plain(self.info_frame.move_dir(Point::new(0, -1)).location(), "Info");

        self.draw_box(console, self.message_frame.move_dir(Point::new(-1, -1)).resize(Size::new(1, 1)));
        console.print_plain(self.message_frame.move_dir(Point::new(0, -1)).location(), "Messages");
    }

    fn draw_box(&self, console: &mut Console, rect: Rectangle) {
        console.put_plain(rect.location() + Point::new(0, 0), chars::NW);
        console.put_plain(rect.location() + Point::new(rect.width(), 0), chars::NE);
        console.put_plain(rect.location() + Point::new(0, rect.height()), chars::SW);
        console.put_plain(rect.location() + Point::new(rect.width(), rect.height()), chars::SE);

        for x in 1..rect.width() {
            console.put_plain(rect.location() + Point::new(x, 0), chars::HLINE);
            console.put_plain(rect.location() + Point::new(x, rect.height()), chars::HLINE);
        }

        for y in 1..rect.height() {
            console.put_plain(rect.location() + Point::new(0, y), chars::VLINE);
            console.put_plain(rect.location() + Point::new(rect.width(), y), chars::VLINE);
        }
    }

    #[allow(unused)]
    fn draw_map(&self, game: &mut Game, console: &mut Console) {
        let map = &game.world.map;

        let (width, height) = (63, 38);

        for (y, line) in map.tiles[self.map_view.y as usize .. self.map_view.y as usize + height].iter().enumerate() {
            for (x, cell) in line[self.map_view.x as usize .. self.map_view.x as usize + width].iter().enumerate() {
                console.put(self.map_frame.location() + Point::new(x as i32, y as i32), ' ', Colors::white, cell.b_color());
            }
        }
    }

    #[allow(unused)]
    fn draw_player(&mut self, game: &mut Game, console: &mut Console) {
        let repr = game.world.player.repr;
        let pos = game.world.player.pos;

        let adjusted_pos = pos + self.map_frame.location() - self.map_view;

        if adjusted_pos.x >= self.map_frame.location().x + self.map_frame.width() - 10 {
            if self.view_can_move_right(game) {
                self.map_view = self.map_view.right(1);
            }
        }

        if adjusted_pos.y >= self.map_frame.location().y + self.map_frame.height() - 5 {
            if self.view_can_move_down(game) {
                self.map_view = self.map_view.down(1);
            }
        }

        if adjusted_pos.y <= self.map_frame.location().y + 5 {
            if self.view_can_move_up(game) {
                self.map_view = self.map_view.up(1);
            }
        }

        if adjusted_pos.x <= self.map_frame.location().x + 10 {
            if self.view_can_move_left(game) {
                self.map_view = self.map_view.left(1);
            }
        }

        if adjusted_pos.x >= self.map_frame.location().x && adjusted_pos.y >= self.map_frame.location().y {
            console.put_plain(self.map_frame.location() - self.map_view + pos, repr);
        }
    }

    #[allow(unused)]
    fn view_can_move_up(&self, game: &Game) -> bool {
        self.map_view.y  > 0
    }

    #[allow(unused)]
    fn view_can_move_down(&self, game: &Game) -> bool {
        self.map_view.y + self.map_frame.height() < game.world.map.height()
    }

    #[allow(unused)]
    fn view_can_move_left(&self, game: &Game) -> bool {
        self.map_view.x > 0
    }

    #[allow(unused)]
    fn view_can_move_right(&self, game: &Game) -> bool {
        self.map_view.x + self.map_frame.width() < game.world.map.width()
    }

}
