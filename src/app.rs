pub mod events;

use std::io::{Write, Read};
use std::sync::mpsc;
use std::rc::Rc;

use crate::frontend::screen::*;
use crate::frontend::repo_selection::RepoSelectionScreen;
use crate::frontend::repo_selection_handler::RepoSelectionHandler;
use crate::backend::pr::*;

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
                Some(Ok(b'q')) => break,
                Some(Ok(input)) => {
                    if current_screen_handler.validate_input(input) {
                        current_screen_handler.process_input(input);
                    }
                },
                _ => (),
            }

            current_screen_handler.update(rect);

            if let Some(evt) = self.event_listener.try_recv().ok() {
                match evt {
                    //Switch to the main screen
                    AppEvent::RepoChosen(number) => break,
                }
            }
        }

        write!(self.buff_out, "{}", termion::cursor::Show).unwrap();
        Ok(())
    }
}
