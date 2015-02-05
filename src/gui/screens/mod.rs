use engine::Game;
use gui::Console;

pub use self::menu_screen::MenuScreen;
pub use self::game_screen::GameScreen;

pub trait Screen {
    fn input(&self, &mut Game, &mut Console) -> Option<ScreenChange>;
    fn update(&self, &mut Game, &mut Console) -> Option<ScreenChange>;
    fn render(&self, &mut Game, &mut Console);
}

pub enum ScreenChange {
    AddScreen(Box<Screen + 'static>),
    RemoveScreen,
    ExitGame,
}

mod menu_screen;
mod game_screen;
