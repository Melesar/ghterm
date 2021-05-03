use crate::app::events::AppEvent;
use super::screen::*;

use std::sync::mpsc;
use std::io::Write;
use std::marker::PhantomData;

use super::screen::{DrawableScreen, InteractableScreen};

pub struct MainScreen <W: Write> {
    event_sender: mpsc::Sender<AppEvent>,
    _marker: PhantomData<W>,
}

impl <W: Write> MainScreen<W> {
    pub fn new (event_sender: mpsc::Sender<AppEvent>) -> Self {
        MainScreen{event_sender, _marker: PhantomData}
    }
}

impl <W: Write> DrawableScreen <W> for MainScreen <W> {
    fn draw(&self, buffer: &mut W, rect: Rect) {
        write!(buffer, "{}", termion::clear::All).unwrap();
        buffer.flush().unwrap();
    }
}

impl <W: Write> InteractableScreen for MainScreen <W> {
    fn validate_input(&self, input: u8) -> bool {
        false
    }

    fn process_input(&mut self, input: u8) {
        
    }
}
