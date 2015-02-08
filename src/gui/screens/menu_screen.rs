use engine::Game;
use gui::{Console, Key, Menu};
use gui::screens::{self, Screen, ScreenChange};
use util::units::Point;

use std::fmt;

#[allow(missing_copy_implementations)]
pub struct MenuScreen {
    menu: Menu<MenuItem>,
}

enum MenuItem {
    StartGame,
    Exit,
}

impl fmt::Display for MenuItem {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MenuItem::StartGame => write!(fmt, "Start Game"),
            MenuItem::Exit => write!(fmt, "Exit Game"),
        }
    }
}

impl MenuScreen {
    pub fn new() -> Box<Screen + 'static> {
        Box::new(
            MenuScreen {
                menu: Menu::new(vec![MenuItem::StartGame, MenuItem::Exit]),
            }
        )
    }
}

impl Screen for MenuScreen {

    #[allow(unused)]
    fn input(&mut self, game: &mut Game, console: &mut Console) -> Option<ScreenChange> {
        if let Some(key) = console.check_for_keypress() {
            match key {
                Key::Up => {
                    self.menu.prev();
                }
                Key::Down => {
                    self.menu.next();
                }
                Key::Enter => {
                    match *self.menu.selected() {
                        MenuItem::StartGame => return Some(ScreenChange::AddScreen(screens::GameScreen::new())),
                        MenuItem::Exit => return Some(ScreenChange::ExitGame),
                    }
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
        let logo_loc = Point::new(0, 0);
        console.print_plain(logo_loc + (0, 0), "                                    ");
        console.print_plain(logo_loc + (0, 1), "   ##### ##### #     #  #   # ##### ");
        console.print_plain(logo_loc + (0, 2), "   #   # #     #     #  #  #  #     ");
        console.print_plain(logo_loc + (0, 3), "   #   # #     #     #  # #   #     ");
        console.print_plain(logo_loc + (0, 4), "   ##### ##### #     #  ##    ###   ");
        console.print_plain(logo_loc + (0, 5), "   # #       # #     #  # #   #     ");
        console.print_plain(logo_loc + (0, 6), "   #  #      # #     #  #  #  #     ");
        console.print_plain(logo_loc + (0, 7), "   #   # ##### ##### #  #   # ##### ");
        console.print_plain(logo_loc + (0, 8), "                                    ");

        for (i, menu_item) in self.menu.enum_items() {
            console.print_plain(Point::new(20, 20) + (0, i as i32), format!("{}", menu_item).as_slice());
            if self.menu.is_selected(i) {
                console.put_plain(Point::new(20, 20) + (-2, i as i32), '*');
            }
        }
    }
}
