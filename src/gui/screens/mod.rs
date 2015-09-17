use engine::Game;
use gui::Console;

mod game_screen;
mod inventory_screen;
mod menu_screen;
mod pause_screen;

pub use self::game_screen::GameScreen;
pub use self::inventory_screen::InventoryScreen;
pub use self::menu_screen::MenuScreen;
pub use self::pause_screen::PauseScreen;

pub trait Screen {
    fn input(&mut self, &mut Game, &mut Console) -> Option<ScreenChange>;
    fn update(&mut self, &mut Game, &mut Console) -> Option<ScreenChange>;
    fn render(&mut self, &mut Game, &mut Console);
}

pub enum ScreenChange {
    AddScreen(Box<Screen>),
    RemoveScreen,
    ExitGame,
}
