use engine::Game;
use gui::{Console, Key};
use gui::screens::{self, Screen, ScreenChange};
use util::units::Point;

use std::fmt::{self, Display};

#[allow(missing_copy_implementations)]
pub struct MenuScreen {
    menu_items: Vec<MenuItem>,
    selected: usize,
}

#[derive(Debug)]
enum MenuItem {
    StartGame,
    Exit,
}

impl Display for MenuItem {
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
                menu_items: vec![MenuItem::StartGame, MenuItem::Exit],
                selected: 0,
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
                    if self.selected > 0 { self.selected -= 1; }
                }
                Key::Down => {
                    if self.selected < self.menu_items.len() - 1 { self.selected += 1; }
                }
                Key::Enter => {
                    let action = &self.menu_items[self.selected];
                    match *action {
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
        return None;
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

        for (i, menu_item) in self.menu_items.iter().enumerate() {
            console.print_plain(Point::new(20, 20) + (0, i as i32), format!("{}", menu_item).as_slice());
            if self.selected == i {
                console.put_plain(Point::new(20, 20) + (-2, i as i32), '*');
            }
        }
    }
}
