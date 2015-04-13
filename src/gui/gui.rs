use engine::Game;
use gui::Console;
use gui::screens::{self, Screen, ScreenChange};
use util::units::Size;

#[derive(PartialEq)]
pub enum State {
    Running,
    Exited
}

pub struct GUI {
    pub console: Console,
    pub screens: Vec<Box<Screen>>,
    game: Game,
    state: State,
}

impl GUI {
    pub fn new(game: Game, console: Console) -> GUI {
        GUI {
            game: game,
            console: console,
            screens: vec![screens::MenuScreen::new()],
            state: State::Running,
        }
    }

    pub fn size(&self) -> Size {
        self.console.size()
    }

    pub fn run(&mut self) {
        while !self.exited() {
            self.render();
            self.update();
            self.handle_input();
        }
    }

    fn handle_input(&mut self) {
        let outcome = self.screens.first_mut()
                                  .expect("No screen to display")
                                  .input(&mut self.game, &mut self.console);
        self.update_screens(outcome);
    }

    fn update(&mut self) {
        let outcome = self.screens.first_mut()
                                  .expect("No screen to display")
                                  .update(&mut self.game, &mut self.console);
        self.update_screens(outcome);
    }

    fn render(&mut self) {
        self.console.clear();
        self.screens.first_mut()
                    .expect("No screen to display")
                    .render(&mut self.game, &mut self.console);
        self.console.flush();
    }

    fn update_screens(&mut self, outcome: Option<ScreenChange>) {
        match outcome {
            Some(ScreenChange::AddScreen(screen)) => { self.screens.insert(0, screen) },
            Some(ScreenChange::RemoveScreen) => { self.screens.remove(0); }
            Some(ScreenChange::ExitGame) => { self.state = State::Exited },
            None => {}
        }
    }

    pub fn exited(&self) -> bool {
        self.console.window_closed() || self.state == State::Exited
    }
}
