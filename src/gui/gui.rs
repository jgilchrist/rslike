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
    pub screens: Vec<Box<Screen + 'static>>,
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
            screens: vec!(screens::MenuScreen::new()),
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
        match self.screens[0].input(&mut self.game, &mut self.console) {
            Some(ScreenChange::AddScreen(screen)) => { self.screens.insert(0, screen) },
            Some(ScreenChange::RemoveScreen) => { self.screens.remove(0); }
            Some(ScreenChange::ExitGame) => { self.state = State::Exited },
            None => {}
        }
    }

    fn update(&mut self) {
        self.screens[0].update(&mut self.game, &mut self.console);
    }

    fn render(&mut self) {
        self.console.clear();
        self.screens[0].render(&mut self.game, &mut self.console);
        self.console.flush();
    }

    pub fn exited(&self) -> bool {
        self.state == State::Exited
    }
}
