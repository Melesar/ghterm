use std::io::Write;

use super::screen::*;

pub struct ConversationTab  {
}

impl  ConversationTab {
    pub fn new () -> Self {
        ConversationTab{}
    }
}

impl  DrawableScreen for ConversationTab {
    fn draw(&self, buffer: &mut dyn Write, rect: Rect) {
        
    }
}

impl  InteractableScreen for ConversationTab {
    fn validate_input(&self, input: u8) -> bool {
        false
    }

    fn process_input(&mut self, input: u8) {
        
    }
}

impl  ApplicationScreen for ConversationTab {
}
