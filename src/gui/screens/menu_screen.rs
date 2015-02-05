use engine::Game;
use gui::{Console, Key};
use gui::screens::{self, Screen, ScreenChange};
use util::units::Point;

#[allow(missing_copy_implementations)]
pub struct MenuScreen;

impl MenuScreen {
    pub fn new() -> Box<Screen + 'static> {
        Box::new(MenuScreen)
    }
}

impl Screen for MenuScreen {

    fn input(&self, game: &mut Game, console: &mut Console) -> Option<ScreenChange> {
        if let Some(key) = console.check_for_keypress() {
            match key {
                Key::Escape => {
                    return Some(ScreenChange::ExitGame);
                },
                Key::Enter => {
                    return Some(ScreenChange::AddScreen(screens::GameScreen::new()));
                },
                _ => {}
            }
        }

        None
    }

    fn update(&self, game: &mut Game, console: &mut Console) -> Option<ScreenChange> {
        return None;
    }

    fn render(&self, game: &mut Game, console: &mut Console) {
        console.put_plain(Point::new(0, 0), 'M');
        console.put_plain(Point::new(1, 0), 'e');
        console.put_plain(Point::new(2, 0), 'n');
        console.put_plain(Point::new(3, 0), 'u');
    }
}
