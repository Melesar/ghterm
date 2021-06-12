pub mod events;

use std::io::{Write, Read};
use std::sync::mpsc;

use crate::frontend::screen::*;
use crate::frontend::repo_selection_handler::RepoSelectionHandler;
use crate::frontend::main_screen_handler::MainScreenHandler;
use crate::backend::task::*;
use crate::backend::gh::GhClient;

use events::AppEvent;

pub struct App<R: Read, W: Write>  {
    buff_out: W,
    buff_in: R,
    gh_client: GhClient,
    event_listener: mpsc::Receiver<AppEvent>,
    sender: mpsc::Sender<AppEvent>,
}

impl<R: Read, W: Write> App<R, W> {

    pub fn new(buff_out: W, buff_in: R, gh_client: GhClient) -> Self {
        let (sender, event_listener) = mpsc::channel::<AppEvent>();
        App {buff_out, buff_in, gh_client, event_listener, sender}
    }

    pub fn run(mut self) -> Result<(), std::io::Error> {
        write!(self.buff_out, "{}", termion::cursor::Hide).unwrap(); 

        let size = termion::terminal_size().unwrap();
        let rect = Rect{x: 0, y: 0, w: size.0, h: size.1};

        let mut task_manager = TaskManager::new();
        let mut current_screen_handler : Box<dyn ScreenHandler> = Box::new(RepoSelectionHandler::new(self.sender.clone(), &mut task_manager, &mut self.gh_client));
        self.sender.send(AppEvent::ScreenRepaint).unwrap();

        let mut input = self.buff_in.bytes();
        loop {
            match input.next() {
                Some(Ok(input)) => {
                    if current_screen_handler.validate_input(input) {
                       current_screen_handler.process_input(input);
                    } else {
                        match input {
                            b'q' => break,
                            _ => (),
                        }
                    }
                },
                _ => (),
            }

            current_screen_handler.update();

            if let Some(evt) = self.event_listener.try_recv().ok() {
                match evt {
                    AppEvent::RepoChosen(number) => {
                        write!(self.buff_out, "{}", termion::clear::All).unwrap(); 
                        current_screen_handler = Box::new(MainScreenHandler::new(number, self.sender.clone(), &self.gh_client));
                        self.sender.send(AppEvent::ScreenRepaint).unwrap();
                    },
                    AppEvent::Error(message) => crate::logs::log(&format!("ERROR: {}", message)), //TODO handle the error
                    AppEvent::ScreenRepaint => current_screen_handler.draw(&mut self.buff_out, rect),
                }
            }
        }

        write!(self.buff_out, "{}", termion::cursor::Show).unwrap();
        Ok(())
    }
}
