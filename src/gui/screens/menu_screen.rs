use engine::Game;
use gui::{Console, Key, Menu};
use gui::screens::{self, Screen, ScreenChange};
use util::units::Point;

use std::fmt;

#[allow(missing_copy_implementations)]
pub struct MenuScreen {
    menu: Menu<MenuItem>,
}

static LOGO: &'static str =
       "                                 \n\
        ##### ##### #     #  #   # ##### \n\
        #   # #     #     #  #  #  #     \n\
        #   # #     #     #  # #   #     \n\
        ##### ##### #     #  ##    ###   \n\
        # #       # #     #  # #   #     \n\
        #  #      # #     #  #  #  #     \n\
        #   # ##### ##### #  #   # ##### \n";

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
    pub fn new() -> Box<Screen> {
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
        let logo_loc = Point::new(24, 20);
        console.print_plain(logo_loc, LOGO);

        let menu_loc = Point::new(33, 30);

        for (i, menu_item) in self.menu.enum_items() {
            console.print_plain(menu_loc.right(2).down(i as i32), &format!("{}", menu_item));
            if self.menu.is_selected(i) {
                console.put_plain(menu_loc.down(i as i32), '>');
            }
        }
    }
}
