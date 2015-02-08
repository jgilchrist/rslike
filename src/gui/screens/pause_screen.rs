use engine::Game;
use gui::{Console, Key, Menu};
use gui::screens::{Screen, ScreenChange};
use util::units::Point;

use std::fmt;

pub struct PauseScreen {
    menu: Menu<MenuItem>,
}

enum MenuItem {
    Resume,
    Exit,
}

impl fmt::Display for MenuItem {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MenuItem::Resume => write!(fmt, "Resume"),
            MenuItem::Exit => write!(fmt, "Exit Game"),
        }
    }
}

impl PauseScreen {
    pub fn new() -> Box<Screen + 'static> {
        Box::new(
            PauseScreen {
                menu: Menu::new(vec![MenuItem::Resume, MenuItem::Exit]),
            }
        )
    }
}

impl Screen for PauseScreen {

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
                        MenuItem::Resume => return Some(ScreenChange::RemoveScreen),
                        MenuItem::Exit => return Some(ScreenChange::ExitGame),
                    }
                },
                Key::Escape => return Some(ScreenChange::RemoveScreen),
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
        console.print_plain(Point::new(0, 0), "Paused");

        for (i, menu_item) in self.menu.enum_items() {
            console.print_plain(Point::new(20, 20) + (0, i as i32), format!("{}", menu_item).as_slice());
            if self.menu.is_selected(i) {
                console.put_plain(Point::new(20, 20) + (-2, i as i32), '*');
            }
        }
    }

}
