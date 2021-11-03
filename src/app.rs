pub mod events;

use termion::input::TermRead;

use std::sync::mpsc;

use crate::frontend::screen::*;
use crate::frontend::repo_selection_handler::RepoSelectionHandler;
use crate::frontend::main_screen_handler::MainScreenHandler;
use crate::backend::task::*;
use crate::backend::gh::GhClient;
use crate::error::Error;

use tui::backend::Backend;
use tui::Terminal;

use events::AppEvent;

pub struct App<'a, B: Backend>  {
    terminal: &'a mut Terminal<B>,
    gh_client: GhClient,
    event_listener: mpsc::Receiver<AppEvent>,
    sender: mpsc::Sender<AppEvent>,
}

impl<'a, B: Backend> App<'a, B> {

    pub fn new(terminal: &'a mut Terminal<B>, gh_client: GhClient) -> Self {
        let (sender, event_listener) = mpsc::channel::<AppEvent>();
        App {terminal, gh_client, event_listener, sender}
    }

    pub fn run(mut self, pr_number: Option<u32>) -> Result<(), Error> {

        let mut task_manager = TaskManager::new(self.sender.clone());
        let mut current_screen_handler : Box<dyn ScreenHandler<B>> = if let Some(pr_number) = pr_number {
            Box::new(MainScreenHandler::new(pr_number, self.sender.clone(), &self.gh_client))
        } else {
            Box::new(RepoSelectionHandler::new(self.sender.clone(), &mut task_manager, &mut self.gh_client))
        };
        self.sender.send(AppEvent::ScreenRepaint).unwrap();

        let input_sender = self.sender.clone();
        std::thread::spawn(move || {
            for key in std::io::stdin().keys() {
                if let Some(key) = key.ok() {
                    input_sender.send(AppEvent::Input(key)).unwrap();
                }
            }
        });

        loop {
            if let Some(evt) = self.event_listener.recv().ok() {
                match evt {
                    AppEvent::RepoChosen(number) => {
                        current_screen_handler = Box::new(MainScreenHandler::new(number, self.sender.clone(), &self.gh_client));
                        self.sender.send(AppEvent::ScreenRepaint).unwrap();
                    },

                    AppEvent::Input(key) => {
                        if current_screen_handler.validate_input(key) {
                            current_screen_handler.process_input(key);
                        } else if key == termion::event::Key::Char('q') {
                            break;
                        }
                    },

                    AppEvent::ScreenRepaint => { self.terminal.draw(|f| current_screen_handler.draw(f)).unwrap(); },
                    AppEvent::TaskCompleted => current_screen_handler.update(),

                    AppEvent::Error(message) => crate::logs::log(&format!("ERROR: {}", message)), //TODO handle the error
                }
            }
        }

        Ok(())
    }
}
