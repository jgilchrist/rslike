use engine::{Game};
use gui::screens::{Screen, ScreenChange};
use gui::{primitives};
use gui::{Console, Key, Widget};
use util::units::{Point, Size};

pub struct InventoryScreen {
    list: Widget,
}

impl InventoryScreen {
    pub fn new() -> Box<Screen> {
        Box::new(
            InventoryScreen {
                list: Widget::new(Point::new(0, 0), Size::new(15, 15)),
            }
        )
    }
}

impl Screen for InventoryScreen {
    #[allow(unused)]
    fn input(&mut self, game: &mut Game, console: &mut Console) -> Option<ScreenChange> {
        if let Some(key) = console.check_for_keypress() {
            match key {
                Key::Escape => {
                    return Some(ScreenChange::RemoveScreen);
                },
                Key::Enter => {
                    return Some(ScreenChange::RemoveScreen);
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
        self.draw_borders(game, console);
    }
}

impl InventoryScreen {
    #[allow(unused)]
    fn draw_borders(&self, game: &mut Game, console: &mut Console) {
        primitives::draw_box_with_title(console, "Inventory", self.list.rect);
    }
}
