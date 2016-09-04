use engine::{Game, Command, MessageType, Tile};
use engine::log;
use gui::{primitives};
use gui::{Console, Colors, Key, Widget};
use gui::screens::{self, Screen, ScreenChange};
use util::units::{AsTuple, Direction, Point, Size};

#[allow(missing_copy_implementations)]
pub struct GameScreen {
    map: Widget,
    info: Widget,
    messages: Widget,
    map_view: Point,
}

impl GameScreen {
    pub fn new() -> Box<Screen> {
        let info_widget_location = Point::new(0, 0);
        let map_widget_location = Point::new(15, 0);
        let message_widget_location = Point::new(15, 40);

        Box::new(
            GameScreen {
                map: Widget::new(map_widget_location, Size::new(64, 39)),
                info: Widget::new(info_widget_location, Size::new(14, 49)),
                messages: Widget::new(message_widget_location, Size::new(64, 9)),
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
        self.draw_info(game, console);
        self.draw_map(game, console);
        self.draw_player(game, console);
        self.draw_messages(game, console);
    }
}

impl GameScreen {
    #[allow(unused)]
    fn draw_borders(&self, game: &mut Game, console: &mut Console) {
        primitives::draw_box_with_title(console, "Map", self.map.rect);
        primitives::draw_box_with_title(console, "Info", self.info.rect);
        primitives::draw_box_with_title(console, "Messages", self.messages.rect);
    }

    #[allow(unused)]
    fn draw_info(&self, game: &mut Game, console: &mut Console) {
        return;
    }

    #[allow(unused)]
    fn draw_map(&self, game: &mut Game, console: &mut Console) {
        let map = &game.world.map;

        let (ux, uy) = self.map_view.as_tuple();
        let (width, height) : (usize, usize) = self.map.rect.inner_size().as_tuple();

        for (y, line) in map.tiles[uy .. uy + height + 1].iter().enumerate() {
            for (x, cell) in line[ux .. ux + width + 1].iter().enumerate() {
                let bg_color = match *cell {
                    Tile::Empty => Colors::BLACK,
                    Tile::Wall => Colors::DARKER_GREY,
                    Tile::Floor => Colors::DARKEST_SEPIA,
                };

                self.map.put(console, Point::new(x as i32, y as i32), ' ', Colors::WHITE, bg_color);
            }
        }
    }

    #[allow(unused)]
    fn draw_player(&mut self, game: &mut Game, console: &mut Console) {
        let pos = *game.world.player.pos();

        let adjusted_pos = pos + self.map.rect.inner_location() - self.map_view;

        if adjusted_pos.x >= self.map.rect.inner_location().x + self.map.rect.width() - 10 {
            if self.view_can_move_right(game) {
                self.map_view = self.map_view.right(1);
            }
        }

        if adjusted_pos.y >= self.map.rect.inner_location().y + self.map.rect.height() - 5 {
            if self.view_can_move_down(game) {
                self.map_view = self.map_view.down(1);
            }
        }

        if adjusted_pos.y <= self.map.rect.inner_location().y + 5 {
            if self.view_can_move_up(game) {
                self.map_view = self.map_view.up(1);
            }
        }

        if adjusted_pos.x <= self.map.rect.inner_location().x + 10 {
            if self.view_can_move_left(game) {
                self.map_view = self.map_view.left(1);
            }
        }

        if adjusted_pos.x >= self.map.rect.inner_location().x && adjusted_pos.y >= self.map.rect.inner_location().y {
            self.map.put_plain(console, pos - self.map_view, '@');
        }
    }

    #[allow(unused)]
    fn draw_messages(&mut self, game: &mut Game, console: &mut Console) {
        let nmessages = self.messages.rect.height() as usize;

        log::LOG.with(|w| {
            for (i, msg) in w.borrow().items().take(nmessages).enumerate() {
                let message_color = match *msg.ty() {
                    MessageType::Info => Colors::WHITE,
                    MessageType::Error => Colors::RED,
                };

                self.messages.print(console, Point::new(0, i as i32), msg.text(), message_color, Colors::BLACK);
            }
        });
    }

    #[allow(unused)]
    fn view_can_move_up(&self, game: &Game) -> bool {
        self.map_view.y  > 0
    }

    #[allow(unused)]
    fn view_can_move_down(&self, game: &Game) -> bool {
        self.map_view.y + self.map.rect.height() < game.world.map.height()
    }

    #[allow(unused)]
    fn view_can_move_left(&self, game: &Game) -> bool {
        self.map_view.x > 0
    }

    #[allow(unused)]
    fn view_can_move_right(&self, game: &Game) -> bool {
        self.map_view.x + self.map.rect.width() < game.world.map.width()
    }
}
