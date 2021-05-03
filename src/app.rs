pub mod events;

use std::io::{Write, Read};
use std::sync::mpsc;

use crate::frontend::screen::*;
use crate::frontend::repo_selection_handler::RepoSelectionHandler;

use events::AppEvent;

pub struct App< R: Read, W: Write>  {
    buff_out: W,
    buff_in: R,
    event_listener: mpsc::Receiver<AppEvent>,
    sender: mpsc::Sender<AppEvent>,
}

impl<R: Read, W: Write> App<R, W> {

    pub fn new(buff_out: W, buff_in: R) -> Self {
        let (sender, event_listener) = mpsc::channel::<AppEvent>();
        App {buff_out, buff_in, event_listener, sender}
    }

    pub fn run(mut self) -> Result<(), std::io::Error> {
        write!(self.buff_out, "{}", termion::cursor::Hide).unwrap(); 

        let size = termion::terminal_size().unwrap();
        let rect = Rect{x: 0, y: 0, w: size.0, h: size.1};

        let mut current_screen_handler = Box::new(RepoSelectionHandler::new(&mut self.buff_out, self.sender.clone()));

        let mut input = self.buff_in.bytes();
        loop {
            match input.next() {
                Some(Ok(input)) => {
                    if current_screen_handler.validate_input(input) &&
                       current_screen_handler.process_input(input) {
                        current_screen_handler.update(rect, true);
                    } else {
                        match input {
                            b'q' => break,
                            _ => (),
                        }
                    }
                },
                _ => (),
            }

            current_screen_handler.update(rect, false);

            if let Some(evt) = self.event_listener.try_recv().ok() {
                match evt {
                    AppEvent::RepoChosen(number) =>crate::logs::log(&format!("Selected repo {}", number)) , //TODO switch to the main screen
                    AppEvent::Error(message) => crate::logs::log(&format!("ERROR: {}", message)), //TODO handle the error
                }
            }
        }

        write!(self.buff_out, "{}", termion::cursor::Show).unwrap();
        Ok(())
    }
}
