use std::io::{Write, Read};
use std::collections::HashMap;
use crate::frontend::screen::*;
use crate::frontend::repo_selection::RepoSelectionScreen;

pub struct App<R: Read, W: Write>  {
    buff_out: W,
    buff_in: R,
    screens: HashMap<ScreenType, Box<dyn ApplicationScreen<W>>>,
}

impl<R: Read, W: Write> App<R, W> {

    pub fn new(buff_out: W, buff_in: R) -> Self {
        let mut screens : HashMap<ScreenType, Box<dyn ApplicationScreen<W>>> = HashMap::new();
        screens.insert(ScreenType::RepoSelection, Box::new(RepoSelectionScreen{}));
        App {buff_out, buff_in, screens}
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
        }

        write!(self.buff_out, "{}", termion::cursor::Show).unwrap();
        Ok(())
    }
}
