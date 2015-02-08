use engine::Game;
use gui::{Console, Key};
use gui::screens::{Screen, ScreenChange};
use util::units::Point;

use std::fmt;

pub struct PauseScreen {
    menu_items: Vec<MenuItem>,
    selected: usize,
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
                menu_items: vec![MenuItem::Resume, MenuItem::Exit],
                selected: 0,
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
                    if self.selected > 0 { self.selected -= 1; }
                }
                Key::Down => {
                    if self.selected < self.menu_items.len() - 1 { self.selected += 1; }
                }
                Key::Enter => {
                    let action = &self.menu_items[self.selected];
                    match *action {
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

        for (i, menu_item) in self.menu_items.iter().enumerate() {
            console.print_plain(Point::new(20, 20) + (0, i as i32), format!("{}", menu_item).as_slice());
            if self.selected == i {
                console.put_plain(Point::new(20, 20) + (-2, i as i32), '*');
            }
        }
    }

}
