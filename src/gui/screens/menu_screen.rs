use engine::Game;
use gui::{Console, Key, Menu, MenuOption};
use gui::screens::{self, Screen, ScreenChange};
use util::units::Point;

#[allow(missing_copy_implementations)]
pub struct MenuScreen {
    menu: Menu<MainMenu>,
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

enum MainMenu {
    StartGame,
    Exit,
}

impl MenuScreen {
    pub fn new() -> Box<Screen> {
        Box::new(
            MenuScreen {
                menu: Menu::new(vec![
                                    MenuOption(MainMenu::StartGame, "Start Game"),
                                    MenuOption(MainMenu::Exit, "Exit Game"),
                                ]),
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
                    match *self.menu.selected().option() {
                        MainMenu::StartGame => return Some(ScreenChange::AddScreen(screens::GameScreen::new())),
                        MainMenu::Exit => return Some(ScreenChange::ExitGame),
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

        for (i, menu_option) in self.menu.items().enumerate() {
            console.print_plain(menu_loc.right(2).down(i as i32), menu_option.text());

            if self.menu.is_selected(i) {
                console.put_plain(menu_loc.down(i as i32), '>');
            }
        }
    }
}
