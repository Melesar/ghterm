mod events;

use std::io::{Write, Read};
use std::collections::HashMap;
use std::sync::mpsc;

use crate::frontend::screen::*;
use crate::frontend::repo_selection::RepoSelectionScreen;

use events::AppEvent;

pub struct App<R: Read, W: Write>  {
    buff_out: W,
    buff_in: R,
    screens: HashMap<ScreenType, Box<dyn ApplicationScreen<W>>>,
    event_listener: mpsc::Receiver<AppEvent>,
}

impl<R: Read, W: Write> App<R, W> {

    pub fn new(buff_out: W, buff_in: R) -> Self {
        let (sender, event_listener) = mpsc::channel::<AppEvent>();
        let mut screens : HashMap<ScreenType, Box<dyn ApplicationScreen<W>>> = HashMap::new();
        screens.insert(ScreenType::RepoSelection, Box::new(RepoSelectionScreen{event_sender: sender.clone()}));
        App {buff_out, buff_in, screens, event_listener}
    }

    pub fn run(mut self) -> Result<(), std::io::Error> {
        write!(self.buff_out, "{}", termion::cursor::Hide).unwrap(); 

        let size = termion::terminal_size().unwrap();
        let rect = Rect{x: 0, y: 0, w: size.0, h: size.1};

        let first_screen = self.screens.get(&ScreenType::RepoSelection).unwrap();
        first_screen.draw(&mut self.buff_out, rect);

        self.buff_out.flush().unwrap();
        
        let mut input = self.buff_in.bytes();
        loop {
            let b = input.next();
            if let Some(Ok(b'q')) = b { break; }

            if let Some(evt) = self.event_listener.try_recv().ok() {
                match evt {
                    AppEvent::RepoChosen(number) => break,
                }
            }
        }

        write!(self.buff_out, "{}", termion::cursor::Show).unwrap();
        Ok(())
    }
}
