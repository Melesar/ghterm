use std::io::Write;
use std::marker::PhantomData;

use super::main_screen::MainScreenTab;
use super::screen::*;

pub struct ConversationTab <W: Write> {
    _marker: PhantomData<W>,
}

impl <W: Write> ConversationTab<W> {
    pub fn new () -> Self {
        ConversationTab{_marker: PhantomData}
    }
}

impl <W: Write> MainScreenTab<W> for ConversationTab<W> {
    fn get_title(&self) -> String {
        String::new()
    }
}

impl <W: Write> DrawableScreen<W> for ConversationTab<W> {
    fn draw(&self, buffer: &mut W, rect: Rect) {
        
    }
}

impl <W: Write> InteractableScreen for ConversationTab<W> {
    fn validate_input(&self, input: u8) -> bool {
        false
    }

    fn process_input(&mut self, input: u8) {
        
    }
}

impl <W: Write> ApplicationScreen<W> for ConversationTab<W> {
}
