use engine::{Game, Command, MessageType, Tile};
use gui::screens::{self, Screen, ScreenChange};
use gui::{Console, Colors, Key};
use gui::chars;
use util::units::{AsTuple, Direction, Offset, Point, Rectangle, Size};

#[allow(missing_copy_implementations)]
pub struct GameScreen {
    frames: Frames,
    map_view: Point,
}

struct Frames {
    map: Rectangle,
    info: Rectangle,
    messages: Rectangle,
}

impl GameScreen {
    pub fn new() -> Box<Screen> {
        let info_frame_location = Point::new(1, 1);
        let map_frame_location = Point::new(16, 1);
        let message_frame_location = Point::new(16, 41);

        let frames = Frames {
            map: Rectangle::new(map_frame_location, Size::new(63, 38)),
            info: Rectangle::new(info_frame_location, Size::new(13, 48)),
            messages: Rectangle::new(message_frame_location, Size::new(63, 8)),
        };

        Box::new(
            GameScreen {
                frames: frames,
                map_view: Point::new(0, 0),
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
                    game.do_command(Command::Walk(Direction::Up));
                },
                Key::Down => {
                    game.do_command(Command::Walk(Direction::Down));
                },
                Key::Left => {
                    game.do_command(Command::Walk(Direction::Left));
                },
                Key::Right => {
                    game.do_command(Command::Walk(Direction::Right));
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
        game.step();
        None
    }

    #[allow(unused)]
    fn render(&mut self, game: &mut Game, console: &mut Console) {
        self.draw_borders(game, console);
        self.draw_map(game, console);
        self.draw_player(game, console);
        self.draw_messages(game, console);
    }
}

impl GameScreen {

    #[allow(unused)]
    fn draw_borders(&self, game: &mut Game, console: &mut Console) {
        self.draw_box_with_title(console, "Map", self.frames.map.translate(Offset::new(-1, -1)).resize(Offset::new(1, 1)));
        self.draw_box_with_title(console, "Info", self.frames.info.translate(Offset::new(-1, -1)).resize(Offset::new(1, 1)));
        self.draw_box_with_title(console, "Messages", self.frames.messages.translate(Offset::new(-1, -1)).resize(Offset::new(1, 1)));
    }

    fn draw_box_with_title(&self, console: &mut Console, title: &str, rect: Rectangle) {

        let loc = rect.location();
        let width = rect.width();
        let height = rect.height();

        console.put_plain(loc + (0, 0),             chars::NW);
        console.put_plain(loc + (width, 0),         chars::NE);
        console.put_plain(loc + (0, height),        chars::SW);
        console.put_plain(loc + (width, height),    chars::SE);

        for x in 1..width {
            console.put_plain(loc + (x, 0),         chars::HLINE);
            console.put_plain(loc + (x, height),    chars::HLINE);
        }

        for y in 1..height {
            console.put_plain(loc + (0, y),         chars::VLINE);
            console.put_plain(loc + (width, y),     chars::VLINE);
        }

        console.print_plain(rect.location() + (1, 0), title);
    }

    #[allow(unused)]
    fn draw_map(&self, game: &mut Game, console: &mut Console) {
        let map = &game.world.map;

        let (ux, uy) = self.map_view.as_tuple();
        let (width, height) : (usize, usize) = self.frames.map.size().as_tuple();

        for (y, line) in map.tiles[uy .. uy + height].iter().enumerate() {
            for (x, cell) in line[ux .. ux + width].iter().enumerate() {
                let bg_color = match *cell {
                    Tile::Empty => Colors::BLACK,
                    Tile::Wall => Colors::DARKER_GREY,
                    Tile::Floor => Colors::DARKEST_SEPIA,
                };

                console.put(self.frames.map.location() + Point::new(x as i32, y as i32), ' ', Colors::WHITE, bg_color);
            }
        }
    }

    #[allow(unused)]
    fn draw_player(&mut self, game: &mut Game, console: &mut Console) {
        let pos = *game.world.player.pos();

        let adjusted_pos = pos + self.frames.map.location() - self.map_view;

        if adjusted_pos.x >= self.frames.map.location().x + self.frames.map.width() - 10 {
            if self.view_can_move_right(game) {
                self.map_view = self.map_view.right(1);
            }
        }

        if adjusted_pos.y >= self.frames.map.location().y + self.frames.map.height() - 5 {
            if self.view_can_move_down(game) {
                self.map_view = self.map_view.down(1);
            }
        }

        if adjusted_pos.y <= self.frames.map.location().y + 5 {
            if self.view_can_move_up(game) {
                self.map_view = self.map_view.up(1);
            }
        }

        if adjusted_pos.x <= self.frames.map.location().x + 10 {
            if self.view_can_move_left(game) {
                self.map_view = self.map_view.left(1);
            }
        }

        if adjusted_pos.x >= self.frames.map.location().x && adjusted_pos.y >= self.frames.map.location().y {
            console.put_plain(self.frames.map.location() - self.map_view + pos, '@');
        }
    }

    #[allow(unused)]
    fn draw_messages(&mut self, game: &mut Game, console: &mut Console) {
        let nmessages = self.frames.messages.height() as usize;

        for (i, msg) in game.log.items().take(nmessages).enumerate() {
            let message_color = match *msg.ty() {
                MessageType::Info => Colors::WHITE,
                MessageType::Error => Colors::RED,
            };

            console.print(self.frames.messages.location().down(i as i32), msg.text(), message_color, Colors::BLACK);
        }
    }

    #[allow(unused)]
    fn view_can_move_up(&self, game: &Game) -> bool {
        self.map_view.y  > 0
    }

    #[allow(unused)]
    fn view_can_move_down(&self, game: &Game) -> bool {
        self.map_view.y + self.frames.map.height() < game.world.map.height()
    }

    #[allow(unused)]
    fn view_can_move_left(&self, game: &Game) -> bool {
        self.map_view.x > 0
    }

    #[allow(unused)]
    fn view_can_move_right(&self, game: &Game) -> bool {
        self.map_view.x + self.frames.map.width() < game.world.map.width()
    }

}
