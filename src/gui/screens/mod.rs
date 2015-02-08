use engine::Game;
use gui::Console;

pub use self::menu_screen::MenuScreen;
pub use self::game_screen::GameScreen;

pub trait Screen {
    fn input(&mut self, &mut Game, &mut Console) -> Option<ScreenChange>;
    fn update(&mut self, &mut Game, &mut Console) -> Option<ScreenChange>;
    fn render(&mut self, &mut Game, &mut Console);
}

pub enum ScreenChange {
    AddScreen(Box<Screen + 'static>),
    RemoveScreen,
    ExitGame,
}

mod menu_screen;
mod game_screen;
