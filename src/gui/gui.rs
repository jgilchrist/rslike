use engine::Game;
use gui::Console;
use gui::screens;
use util::units::Size;

#[derive(PartialEq)]
pub enum State {
    Running,
    Exited
}

pub struct GUI {
    pub console: Console,
    pub current_screen: Box<Screen + 'static>,
    game: Game,
    state: State,
}

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

impl GUI {
    pub fn new(game: Game, console: Console) -> GUI {
        GUI {
            game: game,
            console: console,
            current_screen: screens::GameScreen::new(),
            state: State::Running,
        }
    }

    pub fn size(&self) -> Size {
        self.console.size()
    }

    pub fn run(&mut self) {
        while !self.console.window_closed() && !self.exited() {
            self.render();
            self.update();
            self.handle_input();
        }
    }

    fn handle_input(&mut self) {
        if let Some(ScreenChange::ExitGame) = self.current_screen.input(&mut self.game, &mut self.console) {
            self.state = State::Exited;
        }
    }

    fn update(&mut self) {
        self.current_screen.update(&mut self.game, &mut self.console);
    }

    fn render(&mut self) {
        self.console.clear();
        self.current_screen.render(&mut self.game, &mut self.console);
        self.console.flush();
    }

    pub fn exited(&self) -> bool {
        self.state == State::Exited
    }
}
